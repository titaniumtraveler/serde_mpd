use crate::{Error, Result};

pub trait Read<'de> {
    fn peek(&mut self) -> Result<Option<u8>>;
    fn next(&mut self) -> Result<Option<u8>>;
    fn discard(&mut self);

    fn read_until<'s>(
        &mut self,
        scratch: &'s mut Vec<u8>,
        until: u8,
    ) -> Result<Reference<'de, 's, [u8]>>;
    fn read_len<'s>(
        &mut self,
        scratch: &'s mut Vec<u8>,
        len: usize,
    ) -> std::result::Result<Reference<'de, 's, [u8]>, (usize, Error)>;

    fn skip_until(&mut self, until: u8) -> Result<()>;

    fn starts_with(&mut self, scratch: &mut Vec<u8>, starts_with: &str) -> Result<bool>;
}

pub enum Reference<'de, 's, T>
where
    T: ?Sized + 'static,
{
    Borrowed(&'de T),
    Copied(&'s T),
}
