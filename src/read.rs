use crate::{Error, Result};

pub use self::slice::Slice;

mod slice;

pub trait Read<'de> {
    fn peek(&mut self) -> Result<Option<u8>>;
    fn next(&mut self) -> Result<Option<u8>>;
    fn discard(&mut self);

    /// Reads in bytes until the `until` byte is encountered and then returns a [`Reference`] to it.
    /// After usage of the [`Reference`] the scratch buffer has to be [`Vec::clear()`]ed.
    fn read_until<'s>(
        &mut self,
        scratch: &'s mut Vec<u8>,
        until: u8,
    ) -> Result<Reference<'de, 's, [u8]>>;
    /// Reads in len bytes and then returns a [`Reference`] to it.
    /// After usage of the [`Reference`] the scratch buffer has to be [`Vec::clear()`]ed.
    fn read_len<'s>(
        &mut self,
        scratch: &'s mut Vec<u8>,
        len: usize,
    ) -> std::result::Result<Reference<'de, 's, [u8]>, (usize, Error)>;

    fn skip_until(&mut self, until: u8) -> Result<()>;

    /// Checks if input starts with `starts_with` and discards those bytes if yes.
    /// Might use scratch to buffer the bytes in scratch and will [`Vec::clear()`] it itself in
    /// case of a positive result. (`Ok(true)`)
    fn starts_with(&mut self, scratch: &mut Vec<u8>, starts_with: &[u8]) -> Result<bool>;
}

pub enum Reference<'de, 's, T>
where
    T: ?Sized + 'static,
{
    Borrowed(&'de T),
    Copied(&'s T),
}
