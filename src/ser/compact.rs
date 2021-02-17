use crate::error::Error;
use crate::escape::{escape, escape_cold, need_escape};
use crate::float::{FiniteF32, FiniteF64};
use crate::raw::RawStr;
use crate::suffix::{MapSuffix, RootSuffix, SeqSuffix, Suffix};
use crate::{bufwrite::BufWrite, escape::escape_char};

use serde::ser;
use std::marker::PhantomData;

macro_rules! imap {
    ($expr:expr) => {
        match $expr {
            Ok(r) => Ok(r),
            Err(e) => Err(Error::Io(e)),
        }
    };
}

#[derive(Debug)]
pub(crate) struct Serializer<'w, W: BufWrite + 'w, S: Suffix = RootSuffix> {
    writer: &'w mut W,
    _suffix: PhantomData<S>,
}

impl<'w, W: BufWrite> Serializer<'w, W, RootSuffix> {
    #[inline]
    pub(crate) fn new(writer: &'w mut W) -> Self {
        Self {
            writer,
            _suffix: PhantomData,
        }
    }
}

impl<'a, 'w: 'a, W: BufWrite, S: Suffix> ser::Serializer
    for &'a mut Serializer<'w, W, S>
{
    type Ok = ();
    type Error = Error;
    type SerializeSeq = SeqSerializer<'a, W, S>;
    type SerializeTuple = SeqSerializer<'a, W, S>;
    type SerializeTupleStruct = SeqSerializer<'a, W, S>;
    type SerializeTupleVariant = SeqSerializer<'a, W, S>;
    type SerializeMap = MapSerializer<'a, W, S>;
    type SerializeStruct = StructSerializer<'a, W, S>;
    type SerializeStructVariant = StructSerializer<'a, W, S>;

    #[inline]
    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        imap!(self.writer.write2(&v, &RawStr(S::SUFFIX)))
    }

    #[inline]
    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        imap!(self.writer.write2(&v, &RawStr(S::SUFFIX)))
    }

    #[inline]
    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        imap!(self.writer.write2(&v, &RawStr(S::SUFFIX)))
    }

    #[inline]
    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        imap!(self.writer.write2(&v, &RawStr(S::SUFFIX)))
    }

    #[inline]
    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        imap!(self.writer.write2(&v, &RawStr(S::SUFFIX)))
    }

    #[inline]
    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        imap!(self.writer.write2(&v, &RawStr(S::SUFFIX)))
    }

    #[inline]
    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        imap!(self.writer.write2(&v, &RawStr(S::SUFFIX)))
    }

    #[inline]
    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        imap!(self.writer.write2(&v, &RawStr(S::SUFFIX)))
    }

    #[inline]
    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        imap!(self.writer.write2(&v, &RawStr(S::SUFFIX)))
    }

    fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
        imap!(self.writer.write2(&v, &RawStr(S::SUFFIX)))
    }

    fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
        imap!(self.writer.write2(&v, &RawStr(S::SUFFIX)))
    }

    #[inline]
    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        let v = tri!(FiniteF32::new(v).ok_or(Error::NonFiniteFloat));
        imap!(self.writer.write2(&v, &RawStr(S::SUFFIX)))
    }

    #[inline]
    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        let v = tri!(FiniteF64::new(v).ok_or(Error::NonFiniteFloat));
        imap!(self.writer.write2(&v, &RawStr(S::SUFFIX)))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        // TODO: pre-allocate all things
        self.writer.write_all(b"\"")?;
        escape_char(self.writer, v)?;
        imap!(self.writer.write2(&RawStr("\""), &RawStr(S::SUFFIX)))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.writer.write_all(b"\"")?;

        // TODO: implement escape with suffix
        match escape(self.writer, v) {
            Ok(_) => imap!(self.writer.write2(&RawStr("\""), &RawStr(S::SUFFIX))),
            Err(e) => Err(Error::Io(e)),
        }
    }

    #[inline]
    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        // TODO: specialize for byte sequence
        ser::Serialize::serialize(v, self)
    }

    #[inline]
    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    #[inline]
    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ser::Serialize,
    {
        value.serialize(self)
    }

    #[inline]
    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        imap!(self.writer.write2(&RawStr("null"), &RawStr(S::SUFFIX)))
    }

    #[inline]
    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        if !need_escape(variant) {
            imap!(self.writer.write2(&RawStr(variant), &RawStr(S::SUFFIX)))
        } else {
            match escape_cold(self.writer, variant) {
                Ok(_) => {
                    if !S::SUFFIX.is_empty() {
                        imap!(self.writer.write_all(S::SUFFIX.as_bytes()))
                    } else {
                        Ok(())
                    }
                }
                Err(e) => Err(Error::Io(e)),
            }
        }
    }

    #[inline]
    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ser::Serialize,
    {
        value.serialize(self)
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ser::Serialize,
    {
        if !need_escape(variant) {
            self.writer
                .write3(&RawStr("\""), &RawStr(variant), &RawStr("\":"))?;
        } else {
            self.writer.write_all(b"\"")?;
            match escape_cold(self.writer, variant) {
                Ok(_) => {
                    if !S::SUFFIX.is_empty() {
                        self.writer.write_all(b"\":")?
                    }
                }
                Err(e) => return Err(Error::Io(e)),
            }
        }

        value.serialize(self)
    }

    fn serialize_seq(
        self,
        len: Option<usize>,
    ) -> Result<Self::SerializeSeq, Self::Error> {
        let first = len.map_or(true, |len| len == 0);
        self.writer.write_all(b"[")?;
        Ok(SeqSerializer {
            inner: Serializer::<'a, W, SeqSuffix> {
                writer: self.writer,
                _suffix: PhantomData,
            },
            first,
            _suffix: PhantomData,
        })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        let first = len == 0;
        self.writer.write_all(b"[")?;
        Ok(SeqSerializer {
            inner: Serializer::<'a, W, SeqSuffix> {
                writer: self.writer,
                _suffix: PhantomData,
            },
            first,
            _suffix: PhantomData,
        })
    }

    #[inline]
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        <Self as ser::Serializer>::serialize_tuple(self, len)
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        if !need_escape(variant) {
            self.writer
                .write3(&RawStr("\""), &RawStr(variant), &RawStr("\":["))?;
        } else {
            self.writer.write_all(b"\"")?;
            match escape_cold(self.writer, variant) {
                Ok(_) => {
                    if !S::SUFFIX.is_empty() {
                        self.writer.write_all(b"\":[")?
                    }
                }
                Err(e) => return Err(Error::Io(e)),
            }
        }

        Ok(SeqSerializer {
            inner: Serializer::<'a, W, SeqSuffix> {
                writer: self.writer,
                _suffix: PhantomData,
            },
            first: len == 0,
            _suffix: PhantomData,
        })
    }

    fn serialize_map(
        self,
        len: Option<usize>,
    ) -> Result<Self::SerializeMap, Self::Error> {
        let first = len.map_or(true, |len| len == 0);
        self.writer.write_all(b"{\"")?;
        Ok(MapSerializer {
            inner: Serializer::<'a, W, MapSuffix> {
                writer: self.writer,
                _suffix: PhantomData,
            },
            first,
            _suffix: PhantomData,
        })
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        let first = len == 0;
        self.writer.write_all(b"{\"")?;
        Ok(StructSerializer {
            inner: Serializer::<'a, W, MapSuffix> {
                writer: self.writer,
                _suffix: PhantomData,
            },
            first,
            _suffix: PhantomData,
        })
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        if !need_escape(variant) {
            self.writer
                .write3(&RawStr("\""), &RawStr(variant), &RawStr("\":{\""))?;
        } else {
            self.writer.write_all(b"\"")?;
            match escape_cold(self.writer, variant) {
                Ok(_) => {
                    if !S::SUFFIX.is_empty() {
                        self.writer.write_all(b"\":{\"")?
                    }
                }
                Err(e) => return Err(Error::Io(e)),
            }
        }

        Ok(StructSerializer {
            inner: Serializer::<'a, W, MapSuffix> {
                writer: self.writer,
                _suffix: PhantomData,
            },
            first: len == 0,
            _suffix: PhantomData,
        })
    }

    fn collect_seq<I>(self, iter: I) -> Result<Self::Ok, Self::Error>
    where
        I: IntoIterator,
        I::Item: ser::Serialize,
    {
        self.writer.write_all(b"[")?;

        let it = iter.into_iter();
        let mut first = true;
        let mut ser = Serializer {
            writer: self.writer,
            _suffix: PhantomData::<SeqSuffix>,
        };

        for elem in it {
            first = false;
            tri!(ser::Serialize::serialize(&elem, &mut ser));
        }

        if !first {
            // strip last comma
            unsafe {
                self.writer.shrink(1);
            }
        }
        imap!(self.writer.write2(&RawStr("]"), &RawStr(S::SUFFIX)))
    }
}

#[doc(hidden)]
pub struct SeqSerializer<'w, W: BufWrite, S: Suffix> {
    inner: Serializer<'w, W, SeqSuffix>,
    first: bool,
    _suffix: PhantomData<S>,
}

impl<'w, W: BufWrite, S: Suffix> ser::SerializeTuple for SeqSerializer<'w, W, S> {
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        self.first = false;
        value.serialize(&mut self.inner)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        if likely!(!self.first) {
            unsafe {
                self.inner.writer.shrink(SeqSuffix::SUFFIX.len());
            }
        }

        imap!(self.inner.writer.write2(&RawStr("]"), &RawStr(S::SUFFIX)))
    }
}

impl<'w, W: BufWrite, S: Suffix> ser::SerializeTupleStruct for SeqSerializer<'w, W, S> {
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        self.first = false;
        value.serialize(&mut self.inner)
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        <Self as ser::SerializeTuple>::end(self)
    }
}

impl<'w, W: BufWrite, S: Suffix> ser::SerializeTupleVariant for SeqSerializer<'w, W, S> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        self.first = false;
        value.serialize(&mut self.inner)
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        <Self as ser::SerializeTuple>::end(self)
    }
}

impl<'w, W: BufWrite, S: Suffix> ser::SerializeSeq for SeqSerializer<'w, W, S> {
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        self.first = false;
        value.serialize(&mut self.inner)
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        <Self as ser::SerializeTuple>::end(self)
    }
}

#[doc(hidden)]
pub struct StructSerializer<'w, W: BufWrite, S: Suffix> {
    inner: Serializer<'w, W, MapSuffix>,
    first: bool,
    _suffix: PhantomData<S>,
}

impl<'w, W: BufWrite, S: Suffix> ser::SerializeStruct for StructSerializer<'w, W, S> {
    type Ok = ();
    type Error = Error;

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        if !need_escape(key) {
            self.inner.writer.write2(&RawStr(key), &RawStr("\":"))?;
        } else {
            match escape_cold(self.inner.writer, key) {
                Ok(_) => {
                    if !S::SUFFIX.is_empty() {
                        self.inner.writer.write_all(b"\":")?;
                    }
                }
                Err(e) => return Err(Error::Io(e)),
            }
        }

        value.serialize(&mut self.inner)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unsafe {
            self.inner
                .writer
                .shrink(MapSuffix::SUFFIX.len() - self.first as usize);
        }

        imap!(self.inner.writer.write2(&RawStr("}"), &RawStr(S::SUFFIX)))
    }
}

impl<'w, W: BufWrite, S: Suffix> ser::SerializeStructVariant for StructSerializer<'w, W, S> {
    type Ok = ();
    type Error = Error;

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        <Self as ser::SerializeStruct>::serialize_field(self, key, value)
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        <Self as ser::SerializeStruct>::end(self)
    }
}

#[doc(hidden)]
pub struct MapSerializer<'w, W: BufWrite, S: Suffix> {
    inner: Serializer<'w, W, MapSuffix>,
    first: bool,
    _suffix: PhantomData<S>,
}

impl<'w, W: BufWrite, S: Suffix> ser::SerializeMap for MapSerializer<'w, W, S> {
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        key.serialize(MapKeySerializer {
            writer: self.inner.writer,
        })
    }

    #[inline]
    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        value.serialize(&mut self.inner)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unsafe {
            self.inner
                .writer
                .shrink(MapSuffix::SUFFIX.len() - self.first as usize);
        }

        imap!(self.inner.writer.write2(&RawStr("}"), &RawStr(S::SUFFIX)))
    }
}

pub struct MapKeySerializer<'w, W: BufWrite> {
    writer: &'w mut W,
}

impl<'w, W: BufWrite> ser::Serializer for MapKeySerializer<'w, W> {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = ser::Impossible<(), Error>;
    type SerializeTuple = ser::Impossible<(), Error>;
    type SerializeTupleStruct = ser::Impossible<(), Error>;
    type SerializeTupleVariant = ser::Impossible<(), Error>;
    type SerializeMap = ser::Impossible<(), Error>;
    type SerializeStruct = ser::Impossible<(), Error>;
    type SerializeStructVariant = ser::Impossible<(), Error>;

    fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
        Err(Error::InvalidKey)
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        imap!(self.writer.write2(&v, &RawStr("\":")))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        imap!(self.writer.write2(&v, &RawStr("\":")))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        imap!(self.writer.write2(&v, &RawStr("\":")))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        imap!(self.writer.write2(&v, &RawStr("\":")))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        imap!(self.writer.write2(&v, &RawStr("\":")))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        imap!(self.writer.write2(&v, &RawStr("\":")))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        imap!(self.writer.write2(&v, &RawStr("\":")))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        imap!(self.writer.write2(&v, &RawStr("\":")))
    }

    fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
        imap!(self.writer.write2(&v, &RawStr("\":")))
    }

    fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
        imap!(self.writer.write2(&v, &RawStr("\":")))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        let v = tri!(FiniteF32::new(v).ok_or(Error::NonFiniteFloat));
        imap!(self.writer.write2(&v, &RawStr("\":")))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        let v = tri!(FiniteF64::new(v).ok_or(Error::NonFiniteFloat));
        imap!(self.writer.write2(&v, &RawStr("\":")))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        // TODO: pre-allocate all things
        escape_char(self.writer, v)?;
        imap!(self.writer.write_all(b"\":"))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        escape(self.writer, v)?;
        imap!(self.writer.write_all(b"\":"))
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(Error::InvalidKey)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::InvalidKey)
    }

    fn serialize_some<T: ?Sized>(self, _value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        Err(Error::InvalidKey)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::InvalidKey)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(Error::InvalidKey)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Err(Error::InvalidKey)
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        Err(Error::InvalidKey)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        Err(Error::InvalidKey)
    }

    fn serialize_seq(
        self,
        _len: Option<usize>,
    ) -> Result<Self::SerializeSeq, Self::Error> {
        Err(Error::InvalidKey)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(Error::InvalidKey)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(Error::InvalidKey)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(Error::InvalidKey)
    }

    fn serialize_map(
        self,
        _len: Option<usize>,
    ) -> Result<Self::SerializeMap, Self::Error> {
        Err(Error::InvalidKey)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Err(Error::InvalidKey)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(Error::InvalidKey)
    }
}
