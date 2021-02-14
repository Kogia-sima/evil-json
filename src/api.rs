use crate::bufwriter::BufWriter;
use crate::error::Error;
use crate::ser::compact::Serializer;

use serde::ser::Serialize;
use std::io;

pub fn to_vec<T>(value: &T) -> Result<Vec<u8>, Error>
where
    T: Serialize + ?Sized,
{
    let mut vec = Vec::new();
    let mut ser = Serializer::new(&mut vec);
    tri!(value.serialize(&mut ser));
    Ok(vec)
}

pub fn to_string<T>(value: &T) -> Result<String, Error>
where
    T: Serialize + ?Sized,
{
    let vec = tri!(to_vec(value));
    // SAFETY: JSON data contains only valid UTF-8 sequence
    unsafe { Ok(String::from_utf8_unchecked(vec)) }
}

pub fn to_writer<W, T>(writer: W, value: &T) -> Result<(), Error>
where
    W: io::Write,
    T: Serialize,
{
    // We use our self-implemented bufwriter, which is faster than std crate.
    let mut bufwriter = BufWriter::new(writer);
    let mut ser = Serializer::new(&mut bufwriter);
    value.serialize(&mut ser)
}
