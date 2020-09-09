use serde::{Serialize, Deserialize};
use super::guild::GuildMember;
use crate::types::Snowflake;
use super::CachedTypes;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct VoiceState {
    pub guild_id: Option<Snowflake>,
    pub channel_id: Snowflake,
    pub user_id: Snowflake,
    pub member: Option<GuildMember>,
    pub session_id: Snowflake,
    pub deaf: bool,
    pub mute: bool,
    pub self_deaf: bool,
    pub self_mute: bool,
    #[serde(default)]
    pub self_stream: bool,
    pub self_video: bool,
    pub suppress: bool,
}

impl CachedTypes for VoiceState {}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct VoiceRegion {
    pub id: Snowflake,
    pub name: String,
    pub vip: bool,
    pub optimal: bool,
    pub deprecated: bool,
    pub custom: bool,
}