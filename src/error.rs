use core::fmt;
use serde::ser;

#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    // TODO: print location where error happened.
    //       e.g. `Invalid key type at "KeyA.KeyB.KeyC"`
    InvalidKey,
    NonFiniteFloat,
    Custom(String),
    #[cfg(feature = "std")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    Io(std::io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::InvalidKey => f.pad("Invalid key type for JSON"),
            Error::NonFiniteFloat => f.pad("non-finite float is not allowed in JSON."),
            Error::Custom(ref s) => f.pad(s.as_str()),
            #[cfg(feature = "std")]
            Error::Io(ref err) => err.fmt(f),
        }
    }
}

#[cfg(feature = "std")]
impl From<std::io::Error> for Error {
    fn from(source: std::io::Error) -> Error {
        Error::Io(source)
    }
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl std::error::Error for Error {}

impl ser::Error for Error {
    #[cold]
    fn custom<T: fmt::Display>(msg: T) -> Error {
        Error::Custom(msg.to_string())
    }
}
