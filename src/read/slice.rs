use crate::{
    read::{Read, Reference, SliceDebug},
    Error,
};
use std::fmt::{Debug, Formatter};

#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Default, Clone, Copy)]
pub struct Slice<'de> {
    pub src: &'de [u8],
}

impl Debug for Slice<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&SliceDebug(self.src), f)
    }
}

impl<'de> Read<'de> for Slice<'de> {
    fn peek(&mut self) -> crate::Result<Option<u8>> {
        Ok(self.src.first().copied())
    }
    fn next(&mut self) -> crate::Result<Option<u8>> {
        if let [next, rest @ ..] = self.src {
            self.src = rest;
            Ok(Some(*next))
        } else {
            Ok(None)
        }
    }
    fn discard(&mut self) {
        if let [_, rest @ ..] = self.src {
            self.src = rest;
        }
    }

    fn read_until<'s>(
        &mut self,
        _scratch: &'s mut Vec<u8>,
        until: u8,
    ) -> crate::Result<Reference<'de, 's, [u8]>> {
        let (index, _) = self
            .src
            .iter()
            .enumerate()
            .find(|(_, byte)| **byte == until)
            .ok_or(Error::Eof)?;
        Ok(Reference::Borrowed(&self.src[..index]))
    }
    fn read_len<'s>(
        &mut self,
        _scratch: &'s mut Vec<u8>,
        len: usize,
    ) -> std::result::Result<Reference<'de, 's, [u8]>, (usize, crate::Error)> {
        if len <= self.src.len() {
            let (bytes, rest) = self.src.split_at(len);
            self.src = rest;
            Ok(Reference::Borrowed(bytes))
        } else {
            Err((self.src.len(), Error::Eof))
        }
    }

    fn skip_until(&mut self, until: u8) -> crate::Result<()> {
        let (index, _) = self
            .src
            .iter()
            .enumerate()
            .find(|(_, byte)| **byte == until)
            .ok_or(Error::Eof)?;
        let (_, rest) = self.src.split_at(index + 1);
        self.src = rest;
        Ok(())
    }

    fn starts_with(&mut self, _scratch: &mut Vec<u8>, starts_with: &[u8]) -> crate::Result<bool> {
        if self.src.len() < starts_with.len() {
            Ok(false)
        } else {
            if starts_with == &self.src[..starts_with.len()] {
                self.src = &self.src[starts_with.len()..];
                Ok(true)
            } else {
                Ok(false)
            }
        }
    }
}
