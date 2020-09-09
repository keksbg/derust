use serde::{Serialize, Deserialize};
use super::opcodes::OPCode;
use super::builder::IdentifyObject;
use crate::types::user::User;
use crate::types::channel::Channel;
use crate::types::{CachedTypes, guild::{Guild, PartialGuild}};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct GatewayPayload {
    pub op: OPCode, // opcode
    pub d: GatewayPayloadObjects, // payload object
    pub s: Option<u64>, // sequence number
    pub t: Option<String>, // event name
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)] // won't be serialized but it's required because of GatewayPayloadObjects
pub struct ReadyObject {
    pub v: u8, // gateway version, currently 6
    pub user: User, // object of the User that connected
    pub private_channels: Vec<Channel>, // private channels, at first empty
    pub guilds: Vec<PartialGuild>,
    pub session_id: String,
    pub shard: Option<Vec<u16>>
}

pub enum DiscordEvent {
    Ready(ReadyObject),
    GuildCreate(GuildType),
    
}

#[derive(Serialize, Deserialize, Eq, PartialEq)]
pub enum GuildType {
    Full(Guild),
    Partial(PartialGuild)
}

impl CachedTypes for GuildType {}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
pub struct HelloObject {
    pub heartbeat_interval: u64,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
#[serde(untagged)]
pub enum GatewayPayloadObjects {
    Identify(IdentifyObject),
    Hello(HelloObject),
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
#[serde(untagged)]
pub enum GatewayMessages {
    Ready(ReadyObject),
    Others(GatewayPayload),
}