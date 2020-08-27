use serde::{Serialize, Deserialize};
use super::channel::Channel;
use super::error::DerustError;
use crate::API_URL;
use super::channel::CreatePrivateChannelBody;
use crate::types::Snowflake;
use super::CachedTypes;

/// This represents the basic user structure
/// that is returned by the Discord API.
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct User {
    pub(crate) id: Snowflake,
    pub(crate) username: String,
    pub(crate) discriminator: String,
    pub(crate) avatar: String,
    pub(crate) verified: bool,
    pub(crate) email: String,
    pub(crate) flags: i64,
    pub(crate) premium_type: PremiumType,
    pub(crate) public_flags: i64,
}

impl CachedTypes for User {}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum PremiumType {
    None = 0,
    NitroClassic = 1,
    Nitro = 2,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum PublicFlags {
    None = 0,
    Employee = 1 << 0,
    Partner = 1 << 1,
    HypeSquadEvents = 1 << 2,
    BugHunterLevel1 = 1 << 3,
    BraveryHouse = 1 << 6,
    BrillianceHouse = 1 << 7,
    BalanceHouse = 1 << 8,
    EarlySupporter = 1 << 9,
    TeamUser = 1 << 10,
    System = 1 << 12,
    BugHunterLevel2 = 1 << 14,
    VerifiedBot = 1 << 16,
    VerifiedBotDeveloper = 1 << 17,
}

impl User {
    pub async fn create_private_channel(&self) -> Result<Channel, DerustError> {
        let resp = reqwest::Client::new()
            .post(format!("{}/users/@me/channels", API_URL).as_str())
            .json(
                &CreatePrivateChannelBody{ recipient_id: self.clone().id }
            )
            //.headers()
            .send().await;
        match resp {
            Ok(v) => {
                Ok(v.json::<Channel>().await.unwrap())
            },
            Err(_) => {
                Err(DerustError::UnknownError)
            }
        }
    }
}