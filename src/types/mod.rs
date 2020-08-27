//! # Types
//! All the base types that are used throughout
//! the entire library. All objects that are returned
//! by the API will be deserialized into the specified
//! structs and/or serialized when sent back.
//!
//! Each specific `mod` has types that pertains
//! to its own specific type.
//!
//! The most basic and generic types are found in this `mod`,
//! since they are used by all the other types.
//!
//! [`gateway`](gateway/index.html) contains all the types that
//! will be sent and/or received from the gateway. It also contains
//! builders from which you are able to construct basic objects required
//! to initialize a connection to the gateway.
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

pub mod error;
pub mod invite;
pub mod gateway;
pub mod voice;
pub mod role;
pub mod user;
pub mod channel;
pub mod permission;
pub mod message;
pub mod guild;

/// Discord ID type, represented as a u64 since
/// it is returned as a String by the Discord API.
///
/// Try to use it whenever possible.
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
pub struct Snowflake(#[serde(with = "type_string")] pub u64);
/// Automatic conversion from String into a DateTime struct,
/// because the API returns time as a String formatted as
/// per the [ISO8601 standard](https://en.wikipedia.org/wiki/ISO_8601) for both date and time.
///
/// Always use it as it is easier to work with compared to
/// a standard string.
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Timestamp(#[serde(with = "type_string")] pub DateTime<Utc>);

pub(crate) mod type_string {
    use serde::{
        de::{Deserialize, Deserializer, Error as DeError},
        ser::Serializer,
    };
    use std::{fmt::Display, str::FromStr};

    pub fn serialize<T: Display, S: Serializer>(
        value: &T,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serializer.collect_str(value)
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
        where
            T: FromStr,
            T::Err: Display,
            D: Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(DeError::custom)
    }
}

/// Trait for all the types that are cacheable in the library.
///
/// Current cached objects: [`Guild`], [`GuildMember`], [`User`],
/// [`Channel`], [`VoiceState`]
///
/// [`Guild`]: guild::Guild
/// [`GuildMember`]: guild::GuildMember
/// [`User`]: user::User
/// [`Channel`]: channel::Channel
/// [`VoiceState`]: voice::VoiceState
pub trait CachedTypes {}
