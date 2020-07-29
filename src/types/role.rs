use serde::{Deserialize, Serialize};
use crate::types::Snowflake;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Role {
    pub(crate) id: Snowflake,
    pub(crate) name: String,
    pub(crate) color: i32,
    pub(crate) hoist: bool,
    pub(crate) position: i32,
    pub(crate) permissions: i32,
    pub(crate) managed: bool,
    pub(crate) mentionable: bool,
}