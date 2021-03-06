use serde::de::{self, Deserialize, Deserializer, Unexpected};
use serde::ser::{Serialize, Serializer};
use std::fmt::{self, Display};
use std::mem::MaybeUninit;
use std::ptr;
use std::slice;
use std::str::{self, FromStr};

#[derive(Clone, Copy)]
pub struct Empty;

impl Serialize for Empty {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        [(); 0].serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Empty {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = Empty;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("empty array")
            }

            fn visit_seq<V>(self, _: V) -> Result<Empty, V::Error>
            where
                V: de::SeqAccess<'de>,
            {
                Ok(Empty)
            }
        }

        deserializer.deserialize_tuple(0, Visitor)
    }
}

impl simd_json_derive::Serialize for Empty {
    fn json_write<W>(&self, writer: &mut W) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        writer.write_all(b"[]")
    }
}

#[derive(Clone, Copy)]
pub struct Color(u32);

const HEX_LUT: &'static [u8] = b"\
      000102030405060708090A0B0C0D0E0F101112131415161718191A1B1C1D1E1F\
      202122232425262728292A2B2C2D2E2F303132333435363738393A3B3C3D3E3F\
      404142434445464748494A4B4C4D4E4F505152535455565758595A5B5C5D5E5F\
      606162636465666768696A6B6C6D6E6F707172737475767778797A7B7C7D7E7F\
      808182838485868788898A8B8C8D8E8F909192939495969798999A9B9C9D9E9F\
      A0A1A2A3A4A5A6A7A8A9AAABACADAEAFB0B1B2B3B4B5B6B7B8B9BABBBCBDBEBF\
      C0C1C2C3C4C5C6C7C8C9CACBCCCDCECFD0D1D2D3D4D5D6D7D8D9DADBDCDDDEDF\
      E0E1E2E3E4E5E6E7E8E9EAEBECEDEEEFF0F1F2F3F4F5F6F7F8F9FAFBFCFDFEFF";

impl Color {
    fn as_str(self, buf: &mut MaybeUninit<[u8; 6]>) -> &str {
        let buf_len = 6;
        let buf_ptr = buf.as_mut_ptr() as *mut u8;
        let lut_ptr = HEX_LUT.as_ptr();

        let r = ((self.0 & 0xFF0000) >> 15) as isize;
        let g = ((self.0 & 0x00FF00) >> 7) as isize;
        let b = ((self.0 & 0x0000FF) << 1) as isize;

        unsafe {
            ptr::copy_nonoverlapping(lut_ptr.offset(r), buf_ptr, 2);
            ptr::copy_nonoverlapping(lut_ptr.offset(g), buf_ptr.offset(2), 2);
            ptr::copy_nonoverlapping(lut_ptr.offset(b), buf_ptr.offset(4), 2);

            str::from_utf8(slice::from_raw_parts(buf_ptr, buf_len)).unwrap()
        }
    }
}

impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut buf = MaybeUninit::uninit();
        serializer.serialize_str(self.as_str(&mut buf))
    }
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = Color;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("color string")
            }

            fn visit_str<E>(self, value: &str) -> Result<Color, E>
            where
                E: de::Error,
            {
                match u32::from_str_radix(value, 16) {
                    Ok(hex) => Ok(Color(hex)),
                    Err(_) => Err(E::invalid_value(Unexpected::Str(value), &self)),
                }
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}

impl simd_json_derive::Serialize for Color {
    fn json_write<W>(&self, writer: &mut W) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        let mut buf = MaybeUninit::uninit();
        self.as_str(&mut buf).json_write(writer)
    }
}

#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub struct PrimStr<T>(T);

impl<T: Serialize> Serialize for PrimStr<T> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Serialize::serialize(&self.0, serializer)
    }
}

impl<'de, T> Deserialize<'de> for PrimStr<T>
where
    T: Copy + Ord + Display + FromStr,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use std::marker::PhantomData;
        struct Visitor<T>(PhantomData<T>);

        impl<'de, T> de::Visitor<'de> for Visitor<T>
        where
            T: Copy + Ord + Display + FromStr,
        {
            type Value = PrimStr<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("number represented as string")
            }

            fn visit_str<E>(self, value: &str) -> Result<PrimStr<T>, E>
            where
                E: de::Error,
            {
                match T::from_str(value) {
                    Ok(id) => Ok(PrimStr(id)),
                    Err(_) => Err(E::invalid_value(Unexpected::Str(value), &self)),
                }
            }
        }

        deserializer.deserialize_str(Visitor(PhantomData))
    }
}

impl<T: fmt::Display> simd_json_derive::SerializeAsKey for PrimStr<T> {
    fn json_write<W>(&self, writer: &mut W) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        write!(writer, r#""{}""#, self.0)
    }
}

macro_rules! enum_str {
    ($name:ident { $($variant:ident($str:expr), )* }) => {
        #[derive(Clone, Copy)]
        pub enum $name {
            $($variant,)*
        }

        impl $name {
            fn as_str(self) -> &'static str {
                match self {
                    $( $name::$variant => $str, )*
                }
            }
        }

        impl ::serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where S: ::serde::Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }

        impl<'de> ::serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where D: ::serde::Deserializer<'de>,
            {
                struct Visitor;

                impl<'de> ::serde::de::Visitor<'de> for Visitor {
                    type Value = $name;

                    fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        formatter.write_str("unit variant")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<$name, E>
                        where E: ::serde::de::Error,
                    {
                        match value {
                            $( $str => Ok($name::$variant), )*
                            _ => Err(E::invalid_value(::serde::de::Unexpected::Str(value), &self)),
                        }
                    }
                }

                deserializer.deserialize_str(Visitor)
            }
        }

        impl ::simd_json_derive::Serialize for $name {
            #[inline]
            fn json_write<W>(&self, writer: &mut W) -> std::io::Result<()>
                where W: std::io::Write
            {
                self.as_str().json_write(writer)
            }
        }
    }
}
