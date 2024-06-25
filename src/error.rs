use std::{
    fmt::{self, Debug, Display, Formatter},
    io,
};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Custom(String),
    Io(io::Error),
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
