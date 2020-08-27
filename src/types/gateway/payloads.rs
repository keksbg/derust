use serde::{Serialize, Deserialize};
use super::opcodes::OPCode;
use super::builder::IdentifyObject;
use crate::types::user::User;
use crate::types::channel::Channel;
use crate::types::guild::PartialGuild;

#[derive(Serialize, Deserialize)]
pub struct GatewayPayload {
    pub op: OPCode, // opcode
    pub d: GatewayPayloadObjects, // payload object
    pub s: Option<i32>, // sequence number
    pub t: Option<String>, // event name
}

#[derive(Serialize, Deserialize, Eq, PartialEq)] // won't be serialized but it's required because of GatewayPayloadObjects
pub struct ReadyObject {
    v: u8, // gateway version, currently 6
    user: User, // object of the User that connected
    private_channels: Vec<Channel>, // private channels, at first empty
    guilds: PartialGuild,
    session_id: String,
    shard: Option<Vec<u16>>
}

#[derive(Serialize, Deserialize, Eq, PartialEq)]
#[serde(untagged)]
pub enum GatewayPayloadObjects {
    Identify(IdentifyObject),
    Ready(ReadyObject)
}