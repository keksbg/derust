use serde::{Deserialize, Serialize};
use crate::types::Snowflake;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Role {
    pub id: Snowflake,
    pub name: String,
    pub color: i32,
    pub hoist: bool,
    pub position: i32,
    pub permissions: i32,
    pub managed: bool,
    pub mentionable: bool,
}