use std::{
    fmt::{self, Display, Formatter},
    io,
    str::Utf8Error,
};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error(ErrorCode);

#[derive(Debug)]
pub enum ErrorCode {
    Message(String),
    Io(io::Error),
    UnexpectedEof,
    InvalidEnumAccess,

    ExpectedBool,
    ExpectedChar,
    ExpectedUtf8(Utf8Error),
}

impl Error {
    pub fn new(reason: ErrorCode) -> Self {
        Self(reason)
    }

    pub fn would_block() -> Self {
        Error::new(ErrorCode::Io(io::ErrorKind::WouldBlock.into()))
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.0 {
            ErrorCode::Message(msg) => f.write_str(msg),
            ErrorCode::Io(io) => Display::fmt(io, f),
            ErrorCode::UnexpectedEof => f.write_str("unexpected eof"),
            ErrorCode::InvalidEnumAccess => {
                f.write_str("enum variant accesses other than `newtype_variant` are not supported")
            }
            ErrorCode::ExpectedBool => f.write_str("expected bool"),
            ErrorCode::ExpectedUtf8(err) => write!(f, "expected valid utf8: {err}"),
        }
    }
}

impl std::error::Error for Error {}

impl serde::de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self::new(ErrorCode::Message(msg.to_string()))
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::new(ErrorCode::Io(value))
    }
}

impl From<Utf8Error> for Error {
    fn from(value: Utf8Error) -> Self {
        Self::new(ErrorCode::ExpectedUtf8(value))
    }
}
