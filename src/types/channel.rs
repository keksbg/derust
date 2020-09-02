use super::user::User;
use super::permission::PermissionOverwrite;
use serde::{Serialize, Deserialize};
use crate::types::Snowflake;
use super::CachedTypes;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Channel {
    pub(crate) id: Snowflake,
    pub(crate) r#type: ChannelType,
    pub(crate) guild_id: Option<Snowflake>,
    pub(crate) position: Option<u16>,
    pub(crate) permission_overwrites: Option<Vec<PermissionOverwrite>>,
    pub(crate) name: Option<String>,
    pub(crate) topic: Option<String>,
    pub(crate) nsfw: bool,
    pub(crate) last_message_id: Option<Snowflake>,
    pub(crate) bitrate: Option<u32>,
    pub(crate) user_limit: Option<u16>,
    pub(crate) rate_limit_per_user: Option<u16>,
    pub(crate) recipients: Option<Vec<User>>,
    pub(crate) icon: Option<String>,
    pub(crate) owner_id: Option<Snowflake>,
    pub(crate) application_id: Option<Snowflake>,
    pub(crate) parent_id: Option<Snowflake>,
    pub(crate) last_pin_timestamp: Option<String>
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
    pub(crate) id: Snowflake,
    pub(crate) guild_id: Snowflake,
    pub(crate) r#type: ChannelType,
    pub(crate) name: String,
}

#[derive(Serialize, Clone, Debug, Eq, PartialEq)]
pub(crate) struct CreatePrivateChannelBody {
    pub recipient_id: Snowflake
}

