use crate::{error::ErrorCode, Error};
use serde::de::{value::BorrowedStrDeserializer, Deserializer, EnumAccess, VariantAccess};

pub struct NewtypeVariantAccess<D> {
    de: D,
    variant: &'static str,
}

impl<'de, D> EnumAccess<'de> for NewtypeVariantAccess<D>
where
    D: Deserializer<'de, Error = crate::Error>,
{
    type Error = crate::Error;

    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        seed.deserialize(BorrowedStrDeserializer::new(self.variant))
            .map(|val| (val, self))
    }
}

impl<'de, D> VariantAccess<'de> for NewtypeVariantAccess<D>
where
    D: Deserializer<'de, Error = crate::Error>,
{
    type Error = crate::Error;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Err(Error::new(ErrorCode::InvalidEnumAccess))
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        seed.deserialize(self.de)
    }

    fn tuple_variant<V>(self, _len: usize, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::new(ErrorCode::InvalidEnumAccess))
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::new(ErrorCode::InvalidEnumAccess))
    }
}
