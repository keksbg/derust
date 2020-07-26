use super::guild::Guild;
use super::channel::Channel;
use super::user::User;
use serde::{Deserialize};

#[derive(Deserialize, Clone, Debug)]
pub struct Invite {
    pub(crate) code: String,
    pub(crate) guild: Option<Guild>,
    pub(crate) channel: Channel,
    pub(crate) inviter: Option<User>,
    pub(crate) target_user: Option<User>,
    pub(crate) target_user_type: Option<TargetUserType>,
    pub(crate) approximate_presence_count: Option<i32>,
    pub(crate) approximate_member_count: Option<i32>,
}

#[derive(Deserialize, Clone, Debug)]
pub enum TargetUserType {
    Stream = 1,
}

#[derive(Deserialize, Clone, Debug)]
pub struct InviteMetadata {
    pub(crate) uses: i32,
    pub(crate) max_uses: i32,
    pub(crate) max_age: i32,
    pub(crate) temporary: bool,
    pub(crate) created_at: String, // todo: replace with timestamp object
}