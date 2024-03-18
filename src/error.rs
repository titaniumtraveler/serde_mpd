use std::{
    fmt::Display,
    io::{self, ErrorKind::WouldBlock},
};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error(Inner);

#[derive(Debug)]
enum Inner {
    Message(String),
    WouldBlock,
}

impl Inner {
    fn custom<T: Display>(msg: T) -> Self {
        Self::Message(msg.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Inner::Message(msg) => <String as Display>::fmt(&msg, f),
            Inner::WouldBlock => write!(f, "operation would block"),
        }
    }
}

impl std::error::Error for Error {}

impl serde::de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self(Inner::Message(msg.to_string()))
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        match value.kind() {
            WouldBlock => Self(Inner::WouldBlock),
            _ => Self(Inner::custom(value)),
        }
    }
}
