use crate::{
    error::ErrorCode,
    read::{Read, Reference},
    Error,
};

pub struct SliceRead<'de> {
    slice: &'de [u8],
}

impl<'de> SliceRead<'de> {
    pub fn new(slice: &'de [u8]) -> Self {
        Self { slice }
    }
}

impl<'de> Read<'de> for SliceRead<'de> {
    fn next(&mut self) -> crate::Result<Option<u8>> {
        match self.slice {
            [first, rest @ ..] => {
                self.slice = rest;
                Ok(Some(*first))
            }
            [] => Ok(None),
        }
    }

    fn peek(&mut self) -> crate::Result<Option<u8>> {
        Ok(self.slice.first().copied())
    }

    fn discard(&mut self) {
        match self.slice {
            [_, rest @ ..] => {
                self.slice = rest;
            }
            [] => {}
        }
    }

    fn parse_slice_until<'s>(
        &mut self,
        _scratch: &'s mut Vec<u8>,
        until: u8,
    ) -> crate::Result<Reference<'de, 's, [u8]>> {
        let mut index = 0;
        while index < self.slice.len() && self.slice[index] != until {
            index += 1;
        }

        if index == self.slice.len() {
            return Err(Error::new(ErrorCode::UnexpectedEof));
        }

        let slice = &self.slice[0..index];
        self.slice = &self.slice[index..];
        Ok(Reference::Borrowed(slice))
    }

    fn parse_slice_len<'s>(
        &mut self,
        _scratch: &'s mut Vec<u8>,
        len: usize,
    ) -> std::result::Result<Reference<'de, 's, [u8]>, (usize, crate::Error)> {
        if len < self.slice.len() {
            let slice = &self.slice[0..len];
            self.slice = &self.slice[len..];
            Ok(Reference::Borrowed(slice))
        } else {
            Err((0, Error::new(ErrorCode::UnexpectedEof)))
        }
    }
}
