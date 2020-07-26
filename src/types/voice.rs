use serde::{Serialize, Deserialize};
use super::guild::GuildMember;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VoiceState {
    pub(crate) guild_id: Option<String>,
    pub(crate) channel_id: String,
    pub(crate) user_id: String,
    pub(crate) member: Option<GuildMember>,
    pub(crate) session_id: String,
    pub(crate) deaf: bool,
    pub(crate) mute: bool,
    pub(crate) self_deaf: bool,
    pub(crate) self_mute: bool,
    pub(crate) self_stream: Option<bool>,
    pub(crate) self_video: bool,
    pub(crate) suppress: bool,
}

#[derive(Deserialize, Clone, Debug)]
pub struct VoiceRegion {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) vip: bool,
    pub(crate) optimal: bool,
    pub(crate) deprecated: bool,
    pub(crate) custom: bool,
}