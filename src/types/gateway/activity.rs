use super::super::message::Emoji;
use super::super::user::User;
use serde::{Serialize, Deserialize};
use crate::types::{Snowflake, Timestamp};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct PresenceUpdate {
    pub(crate) user: User,
    pub(crate) roles: Vec<Snowflake>,
    pub(crate) game: Activity,
    pub(crate) guild_id: Snowflake,
    pub(crate) status: String,
    pub(crate) activities: Vec<Activity>,
    pub(crate) client_status: ClientStatus,
    pub(crate) premium_since: Option<Timestamp>,
    pub(crate) nick: Option<String>
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Activity {
    pub(crate) name: String,
    pub(crate) r#type: ActivityType,
    pub(crate) url: Option<String>,
    pub(crate) created_at: i32,
    pub(crate) timestamps: Option<ActivityTimestamp>,
    pub(crate) application_id: Option<Snowflake>,
    pub(crate) details: Option<String>,
    pub(crate) state: Option<String>,
    pub(crate) emoji: Option<Emoji>,
    pub(crate) party: Option<ActivityParty>,
    pub(crate) assets: Option<ActivityAssets>,
    pub(crate) secrets: Option<ActivitySecrets>,
    pub(crate) instance: Option<bool>,
    pub(crate) flags: Option<ActivityFlags>
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ActivityTimestamp {
    pub(crate) start: Option<i32>, // unix timestamp
    pub(crate) end: Option<i32>, // unix timestamp
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum ActivityType {
    Game = 0,
    Streaming = 1,
    Listening = 2,
    Custom = 4,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ClientStatus {
    pub(crate) desktop: Option<String>,
    pub(crate) mobile: Option<String>,
    pub(crate) web: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ActivityParty {
    pub(crate) id: Option<Snowflake>,
    pub(crate) size: Vec<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ActivityAssets {
    pub(crate) large_image: Option<String>,
    pub(crate) large_text: Option<String>,
    pub(crate) small_image: Option<String>,
    pub(crate) small_text: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ActivitySecrets {
    pub(crate) join: Option<String>,
    pub(crate) spectate: Option<String>,
    pub(crate) r#match: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum ActivityFlags {
    Instance = 1 << 0,
    Join = 1 << 1,
    Spectate = 1 << 2,
    JoinRequest = 1 << 3,
    Sync = 1 << 4,
    Play = 1 << 5,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct StatusUpdate {
    /// This represents the timestamp since which the status
    /// has been displayed.
    since: Timestamp,
    /// This is the activity object/game that is being displayed.
    game: Activity,
    /// This is the current status of the user. Correct values:
    /// `online`, `dnd`, `idle`, `invisible`, `offline`
    status: String,
    afk: bool,
}