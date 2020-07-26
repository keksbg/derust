use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Role {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) color: i32,
    pub(crate) hoist: bool,
    pub(crate) position: i32,
    pub(crate) permissions: i32,
    pub(crate) managed: bool,
    pub(crate) mentionable: bool,
}