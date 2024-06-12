use crate::{error::ErrorCode, Error};
use serde::{de, forward_to_deserialize_any};

pub use crate::read::{AsyncIoRead, Read, SliceRead};

mod ack_deserializer;
mod data_deserializer;
mod enum_access;
mod map_access;

pub struct Deserializer<R> {
    read: R,
    scratch: Vec<u8>,
}

impl<'de, R: Read<'de>> Deserializer<R> {
    fn peek(&mut self) -> crate::Result<Option<u8>> {
        self.scratch
            .first()
            .map(|byte| Ok(Some(*byte)))
            .unwrap_or_else(|| self.read.peek())
    }

    /// Checks if reader begins with `ident`.
    /// Before consuming bytes from the reader will check against scratch buffer.
    /// If the reader is interupted while parsing or ident does not match
    /// the parsed bytes, they will be put into the scratch buffer so that the next consumer can
    /// continue, as if nothing ever happened.
    fn parse_ident(&mut self, ident: &[u8]) -> crate::Result<bool> {
        if !ident
            .iter()
            .zip(&self.scratch)
            .all(|(expected, actual)| actual == expected)
        {
            return Ok(false);
        }

        if self.scratch.len() < ident.len() {
            for (index, expected) in ident[self.scratch.len()..].iter().enumerate() {
                match self.read.peek() {
                    Ok(Some(byte)) => {
                        if byte == *expected {
                            self.read.discard();
                        } else {
                            // Extend `self.scratch` with the read bytes for next try
                            self.scratch
                                .extend_from_slice(&ident[self.scratch.len()..index]);

                            return Ok(false);
                        }
                    }
                    Ok(None) => {
                        // Extend `self.scratch` with the read bytes
                        self.scratch
                            .extend_from_slice(&ident[self.scratch.len()..index]);

                        return Err(Error::new(ErrorCode::UnexpectedEof));
                    }
                    Err(err) => {
                        // Extend `self.scratch` with the read bytes
                        self.scratch
                            .extend_from_slice(&ident[self.scratch.len()..index]);

                        return Err(err);
                    }
                }
            }
        }

        if ident.len() < self.scratch.len() {
            // Copy bytes from after the read bytes to the start of scratch buffer
            // and truncate, so that only those bytes remain.
            self.scratch.copy_within(ident.len().., 0);
            self.scratch.truncate(self.scratch.len() - ident.len());
        } else {
            self.scratch.clear();
        }

        Ok(true)
    }
}

impl<'a, 'de, R> serde::Deserializer<'de> for &'a mut Deserializer<R>
where
    R: Read<'de>,
{
    type Error = crate::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_enum("", &[], visitor)
    }

    forward_to_deserialize_any! {
        // The top level Deserializer has no
        // support for primitive types
        bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char

        // Neither for strings or bytes
        str string bytes byte_buf
        option identifier tuple tuple_struct

        // Use `seq` at some point with command_list_ok_begin/command_list_end
        // Entries are terminated by `list_ok` and the list ends with `OK`/`ACK`
        seq
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let peek = self
            .peek()?
            .ok_or_else(|| Error::new(ErrorCode::UnexpectedEof))?;

        if peek == b'A' {
            self.parse_ident(b"ACK ")?;
            todo!("implement ACK deserializer")
        }

        todo!("implement OK deserializer")
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    /// Parse the current entry up to and including the next `OK\n`
    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }
}
