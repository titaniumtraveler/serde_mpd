use crate::{Error, Result};

pub use self::{async_io_read::AsyncIoRead, slice_read::SliceRead};

mod async_io_read;
mod slice_read;

pub trait Read<'de> {
    fn next(&mut self) -> Result<Option<u8>>;
    fn peek(&mut self) -> Result<Option<u8>>;

    /// Only valid after a call to peek(). Discards the peeked byte.
    fn discard(&mut self);

    /// Parse up to next occurence of `until`
    /// Scratch buffer is initially empty.
    ///
    /// If operation was interupted with [`Error::would_block()`] it is possible to
    /// just continue the deserialization process by reusing the partially filled scratch buffer.
    fn parse_slice_until<'s>(
        &mut self,
        scratch: &'s mut Vec<u8>,
        until: u8,
    ) -> Result<Reference<'de, 's, [u8]>>;

    /// Parse `len` bytes.
    /// If this is interupted with [`Error::would_block()`],
    /// the deserializer has to remember the amount of bytes already read and continue with the remaining length
    /// and the same partially filled scratch buffer.
    ///
    /// On failure returns amount of bytes read and the error.
    fn parse_slice_len<'s>(
        &mut self,
        scratch: &'s mut Vec<u8>,
        len: usize,
    ) -> std::result::Result<Reference<'de, 's, [u8]>, (usize, Error)>;
}

pub enum Reference<'de, 's, T>
where
    T: ?Sized + 'static,
{
    Borrowed(&'de T),
    Copied(&'s T),
}

impl<'de, 's, T: ?Sized> Reference<'de, 's, T> {
    pub fn as_ref<'a>(&self) -> &'a T
    where
        'de: 'a,
        's: 'a,
    {
        match self {
            Reference::Borrowed(t) | Reference::Copied(t) => t,
        }
    }
}
