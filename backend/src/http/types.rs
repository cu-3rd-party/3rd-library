#![allow(dead_code)] // тут полезный код, но пока не используемый
use chrono::{DateTime, Utc};
use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::Formatter;

const RFC3339_FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.fZ";

#[derive(sqlx::Type)]
pub struct Timestamptz(pub DateTime<Utc>);

impl Serialize for Timestamptz {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let formatted = self.0.format(RFC3339_FORMAT).to_string();
        serializer.serialize_str(&formatted)
    }
}

impl<'de> Deserialize<'de> for Timestamptz {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct StrVisitor;

        impl Visitor<'_> for StrVisitor {
            type Value = Timestamptz;

            fn expecting(&self, f: &mut Formatter) -> std::fmt::Result {
                f.pad("expected string")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                DateTime::parse_from_str(v, RFC3339_FORMAT)
                    .map(|dt| Timestamptz(dt.with_timezone(&Utc)))
                    .map_err(E::custom)
            }
        }

        deserializer.deserialize_str(StrVisitor)
    }
}
