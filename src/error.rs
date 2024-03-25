use std::{
    fmt::{self, Display, Formatter},
    io,
};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error(ErrorCode);

#[derive(Debug)]
pub enum ErrorCode {
    Message(String),
    Io(io::Error),
    UnexpectedEof,
    ExpectedSomeIdent,
    ExpectedNewline,
}

impl Error {
    pub fn error(reason: ErrorCode) -> Self {
        Self(reason)
    }

    pub fn would_block() -> Self {
        Error::error(ErrorCode::Io(io::ErrorKind::WouldBlock.into()))
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.0 {
            ErrorCode::Message(msg) => f.write_str(&msg),
            ErrorCode::Io(io) => Display::fmt(io, f),
            ErrorCode::UnexpectedEof => f.write_str("unexpected eof"),
            ErrorCode::ExpectedSomeIdent => f.write_str("expected ident"),
            ErrorCode::ExpectedNewline => f.write_str("expected newline character"),
        }
    }
}

impl std::error::Error for Error {}

impl serde::de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self::error(ErrorCode::Message(msg.to_string()))
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::error(ErrorCode::Io(value))
    }
}
