use std::{
    fmt::{self, Debug, Display, Formatter},
    io,
};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Error {
    Custom(String),
    Io(Io),
    /// Some IO operation is pending. If this error is thrown it is safe to retry the operation.
    Pending,
    Eof,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use Error::*;
        match self {
            Custom(str) => f.write_str(str),
            Io(io) => Display::fmt(&io, f),
            Pending => f.write_str("io operations are pending"),
            Eof => f.write_str("unexpected end of file"),
        }
    }
}

impl std::error::Error for Error {}
impl serde::de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Error::Custom(msg.to_string())
    }
}

#[repr(transparent)]
pub struct Io(pub io::Error);

impl Debug for Io {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self.0, f)
    }
}
impl Display for Io {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl PartialEq for Io {
    fn eq(&self, other: &Self) -> bool {
        PartialEq::eq(&self.0.kind(), &other.0.kind())
    }
}
impl Eq for Io where io::ErrorKind: Eq {}
impl PartialOrd for Io {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Io {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        Ord::cmp(&self.0.kind(), &other.0.kind())
    }
}
