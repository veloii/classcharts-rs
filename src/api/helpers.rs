use std::fmt;

use serde::{
    de::{Deserializer, Error, Visitor},
    Deserialize,
};

pub fn deserialize_yes_no_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    struct BoolVisitor;

    impl<'de> Visitor<'de> for BoolVisitor {
        type Value = bool;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string containing 'yes' or 'no'")
        }

        fn visit_str<E>(self, value: &str) -> Result<bool, E>
        where
            E: Error,
        {
            match value {
                "yes" => Ok(true),
                "no" => Ok(false),
                _ => Err(E::invalid_value(serde::de::Unexpected::Str(value), &self)),
            }
        }
    }

    deserializer.deserialize_str(BoolVisitor)
}

#[derive(Deserialize, Debug)]
pub struct Empty {}
