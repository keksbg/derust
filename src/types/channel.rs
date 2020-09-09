use super::user::User;
use super::permission::PermissionOverwrite;
use serde::{Serialize, Deserialize};
use crate::types::Snowflake;
use super::CachedTypes;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Channel {
    pub id: Snowflake,
    pub r#type: ChannelType,
    pub guild_id: Option<Snowflake>,
    pub position: Option<u16>,
    pub permission_overwrites: Option<Vec<PermissionOverwrite>>,
    pub name: Option<String>,
    pub topic: Option<String>,
    #[serde(default)]
    pub nsfw: bool,
    pub last_message_id: Option<Snowflake>,
    pub bitrate: Option<u32>,
    pub user_limit: Option<u16>,
    pub rate_limit_per_user: Option<u16>,
    pub recipients: Option<Vec<User>>,
    pub icon: Option<String>,
    pub owner_id: Option<Snowflake>,
    pub application_id: Option<Snowflake>,
    pub parent_id: Option<Snowflake>,
    pub last_pin_timestamp: Option<String>
}

impl CachedTypes for Channel {}

/// All the different channel types that correspond
/// to the integer returned by the Discord API
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum ChannelType {
    /// Text Channel in a Guild
    GuildText = 0,
    /// Direct Message channel
    DirectMessage = 1,
    /// Voice Channel in a Guild
    GuildVoice = 2,
    /// Group Direct Messages
    GroupDirectMessage = 3,
    /// Guild Category/ies or dividers
    GuildCategory = 4,
    /// Guild News channels provided by the Dev license
    GuildNews = 5,
    /// Guild Store channels provided by the Dev license
    GuildStore = 6,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ChannelMention {
    pub id: Snowflake,
    pub guild_id: Snowflake,
    pub r#type: ChannelType,
    pub name: String,
}

#[derive(Serialize, Clone, Debug, Eq, PartialEq)]
pub struct CreatePrivateChannelBody {
    pub recipient_id: Snowflake
}

