use serde::{Serialize, Deserialize};
use serde_json::Value;
use super::opcodes::OPCode;
use super::builder::IdentifyObject;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct GatewayPayload {
    pub op: OPCode,
    pub d: Value,
    pub s: Option<i32>,
    pub t: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum GatewayPayloadObjects {
    Identify(IdentifyObject)
}