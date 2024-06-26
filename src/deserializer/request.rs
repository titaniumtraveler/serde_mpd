use crate::{read::Read, Error};
use serde::{de::Visitor, forward_to_deserialize_any, Deserializer};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct RequestDeserializer<R> {
    read: R,
    scratch: Vec<u8>,
    state: DeserializerState,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
enum DeserializerState {
    #[default]
    None,
    IgnoreAnyAck,
}

impl<'de, R: Read<'de>> Deserializer<'de> for &mut RequestDeserializer<R> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_enum("", &[], visitor)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64
        char str string bytes byte_buf option
        unit unit_struct newtype_struct seq
        tuple tuple_struct map struct identifier
    }

    fn deserialize_enum<V>(
        self,
        _: &'static str,
        _: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.state {
            DeserializerState::None => {
                if self.read.starts_with(&mut self.scratch, b"ACK ")? {
                    self.state = DeserializerState::IgnoreAnyAck;
                    self.read.skip_until(b'\n')?;
                    self.state = DeserializerState::None;
                    return visitor.visit_unit();
                }

                loop {
                    if self.read.starts_with(&mut self.scratch, b"OK\n")? {
                        break visitor.visit_unit();
                    } else {
                        self.read.skip_until(b'\n')?;
                    }
                }
            }
            DeserializerState::IgnoreAnyAck => {
                self.read.skip_until(b'\n')?;
                self.state = DeserializerState::None;
                visitor.visit_unit()
            }
        }
    }
}
