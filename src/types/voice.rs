use serde::{Serialize, Deserialize};
use super::guild::GuildMember;
use crate::types::Snowflake;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct VoiceState {
    pub(crate) guild_id: Option<Snowflake>,
    pub(crate) channel_id: Snowflake,
    pub(crate) user_id: Snowflake,
    pub(crate) member: Option<GuildMember>,
    pub(crate) session_id: Snowflake,
    pub(crate) deaf: bool,
    pub(crate) mute: bool,
    pub(crate) self_deaf: bool,
    pub(crate) self_mute: bool,
    pub(crate) self_stream: Option<bool>,
    pub(crate) self_video: bool,
    pub(crate) suppress: bool,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct VoiceRegion {
    pub(crate) id: Snowflake,
    pub(crate) name: String,
    pub(crate) vip: bool,
    pub(crate) optimal: bool,
    pub(crate) deprecated: bool,
    pub(crate) custom: bool,
}