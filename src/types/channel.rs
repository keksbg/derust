use super::user::User;
use super::permission::PermissionOverride;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Channel {
    pub(crate) id: String,
    pub(crate) r#type: ChannelType,
    pub(crate) guild_id: Option<String>,
    pub(crate) position: Option<u16>,
    pub(crate) permission_overwrites: Option<Vec<PermissionOverride>>,
    pub(crate) name: Option<String>,
    pub(crate) topic: Option<String>,
    pub(crate) nsfw: bool,
    pub(crate) last_message_id: Option<String>,
    pub(crate) bitrate: Option<u16>,
    pub(crate) user_limit: Option<u16>,
    pub(crate) rate_limit_per_user: Option<u16>,
    pub(crate) recipients: Option<Vec<User>>,
    pub(crate) icon: Option<String>,
    pub(crate) owner_id: Option<String>,
    pub(crate) application_id: Option<String>,
    pub(crate) parent_id: Option<String>,
    pub(crate) last_pin_timestamp: Option<String>
}

/// All the different channel types that correspond
/// to the integer returned by the Discord API
#[derive(Serialize, Deserialize, Clone, Debug)]
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChannelMention {
    pub(crate) id: String,
    pub(crate) guild_id: String,
    pub(crate) r#type: ChannelType,
    pub(crate) name: String,
}

#[derive(Serialize, Clone, Debug)]
pub(crate) struct CreatePrivateChannelBody {
    pub recipient_id: String
}

#[derive(Deserialize, Clone, Debug)]
pub enum DerustError {
    UnknownError,
}
