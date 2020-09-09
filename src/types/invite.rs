use super::guild::Guild;
use super::channel::Channel;
use super::user::User;
use serde::{Deserialize};
use crate::types::Snowflake;

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Invite {
    pub code: String,
    pub guild: Option<Guild>,
    pub channel: Channel,
    pub inviter: Option<User>,
    pub target_user: Option<User>,
    pub target_user_type: Option<TargetUserType>,
    pub approximate_presence_count: Option<i32>,
    pub approximate_member_count: Option<i32>,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum TargetUserType {
    Stream = 1,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct InviteMetadata {
    pub uses: i32,
    pub max_uses: i32,
    pub max_age: i32,
    pub temporary: bool,
    pub created_at: Snowflake,
}