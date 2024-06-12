use crate::{
    de::{Deserializer, Read},
    error::ErrorCode,
    read::Reference,
};
use serde::{de::Error, forward_to_deserialize_any};
use std::{fmt::Display, str::FromStr};

pub struct DataDeserializer<'a, R> {
    deserializer: &'a mut Deserializer<R>,
    read_till: u8,
}

impl<'a, 'de, R> DataDeserializer<'a, R>
where
    R: Read<'de>,
{
    /// TODO: Actually use custom parser that parses byte by byte instead
    fn parse_from_str<T>(&mut self) -> crate::Result<T>
    where
        T: FromStr,
        <T as FromStr>::Err: Display,
    {
        std::str::from_utf8(self.parse_bytes()?.as_ref())?
            .parse()
            .map_err(crate::Error::custom)
    }

    fn parse_bytes<'s>(&'s mut self) -> crate::Result<Reference<'de, 's, [u8]>> {
        self.deserializer
            .read
            .parse_slice_until(&mut self.deserializer.scratch, self.read_till)
    }

    fn clear_scratch(&mut self) {
        self.deserializer.scratch.clear();
    }
}

macro_rules! deserialize_number {
    ($method:ident, $visitor:ident) => {
        fn $method<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: serde::de::Visitor<'de>,
        {
            let val = self.parse_from_str()?;
            self.clear_scratch();
            visitor.$visitor(val)
        }
    };
}

impl<'a, 'de, R> serde::Deserializer<'de> for DataDeserializer<'a, R>
where
    R: Read<'de>,
{
    type Error = crate::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_bool<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let bool = match self.parse_bytes()?.as_ref() {
            b"0" => false,
            b"1" => true,
            _ => return Err(crate::Error::new(ErrorCode::ExpectedBool)),
        };
        self.clear_scratch();
        visitor.visit_bool(bool)
    }

    deserialize_number!(deserialize_i8, visit_i8);
    deserialize_number!(deserialize_i16, visit_i16);
    deserialize_number!(deserialize_i32, visit_i32);
    deserialize_number!(deserialize_i64, visit_i64);
    deserialize_number!(deserialize_u8, visit_u8);
    deserialize_number!(deserialize_u16, visit_u16);
    deserialize_number!(deserialize_u32, visit_u32);
    deserialize_number!(deserialize_u64, visit_u64);
    deserialize_number!(deserialize_f32, visit_f32);
    deserialize_number!(deserialize_f64, visit_f64);

    fn deserialize_char<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let str = std::str::from_utf8(self.parse_bytes()?.as_ref())?;

        let mut iter = str.chars();

        let (Some(char), None) = (iter.next(), iter.next()) else {
            return Err(crate::Error::new(ErrorCode::ExpectedChar));
        };

        self.clear_scratch();
        visitor.visit_char(char)
    }

    fn deserialize_str<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.parse_bytes()? {
            Reference::Borrowed(bytes) => {
                let str = std::str::from_utf8(bytes)?;
                let ret = visitor.visit_borrowed_str(str);
                self.clear_scratch();
                ret
            }
            Reference::Copied(bytes) => {
                let str = std::str::from_utf8(bytes)?;
                let ret = visitor.visit_str(str);
                self.clear_scratch();
                ret
            }
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_bytes<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.parse_bytes()? {
            Reference::Borrowed(bytes) => {
                let ret = visitor.visit_borrowed_bytes(bytes);
                self.clear_scratch();
                ret
            }
            Reference::Copied(bytes) => {
                let ret = visitor.visit_bytes(bytes);
                self.clear_scratch();
                ret
            }
        }
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_bytes(visitor)
    }

    forward_to_deserialize_any! {
        option unit_struct struct newtype_struct seq
        tuple tuple_struct map
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_ignored_any(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_bytes(visitor)
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_bytes(visitor)
    }

    fn deserialize_ignored_any<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.parse_bytes()?;
        self.clear_scratch();
        visitor.visit_unit()
    }
}
