use super::user::User;
use serde::{Serialize, Deserialize};
use super::role::Role;
use super::message::Emoji;
use super::voice::VoiceState;
use super::channel::Channel;
use super::gateway::activity::PresenceUpdate;
use super::permission::PermissionOverwrite;
use crate::types::{Snowflake, Timestamp};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Guild {
    pub(crate) id: Snowflake,
    pub(crate) name: String,
    pub(crate) icon: String,
    pub(crate) splash: String,
    pub(crate) discovery_splash: String,
    pub(crate) owner: Option<bool>,
    pub(crate) owner_id: Snowflake,
    pub(crate) permissions: Option<i32>, //todo: replace with proper permissions
    pub(crate) region: String,
    pub(crate) afk_channel_id: Snowflake,
    pub(crate) afk_timeout: i32,
    pub(crate) embed_enabled: Option<bool>,
    pub(crate) embed_channel_id: Option<Snowflake>,
    pub(crate) verification_level: VerificationLevel,
    pub(crate) default_message_notifications: i32,
    pub(crate) explicit_content_filter: ExplicitFilterLevel,
    pub(crate) roles: Vec<Role>,
    pub(crate) emojis: Vec<Emoji>,
    /// See https://discord.com/developers/docs/resources/guild#guild-object-guild-features
    pub(crate) features: Vec<String>,
    pub(crate) mfa_level: MFALevel,
    pub(crate) application_id: Snowflake,
    pub(crate) widget_enabled: Option<bool>,
    pub(crate) widget_channel_id: Option<Snowflake>,
    pub(crate) system_channel_id: Snowflake,
    pub(crate) system_channel_flags: i32,
    pub(crate) rules_channel_id: Snowflake,
    pub(crate) joined_at: Option<Timestamp>, // utc time
    pub(crate) large: Option<bool>,
    pub(crate) unavailable: Option<bool>,
    pub(crate) member_count: Option<i32>,
    pub(crate) voice_states: Option<Vec<VoiceState>>,
    pub(crate) members: Option<Vec<GuildMember>>,
    pub(crate) channels: Option<Vec<Channel>>,
    pub(crate) presences: Option<Vec<PresenceUpdate>>,
    pub(crate) max_presences: Option<i32>,
    pub(crate) max_members: Option<i32>,
    pub(crate) vanity_url_code: String,
    pub(crate) description: String,
    pub(crate) banner: String,
    pub(crate) premium_tier: i32,
    pub(crate) premium_subscription_count: Option<i32>,
    pub(crate) preferred_locale: String,
    pub(crate) public_updates_channel_id: Snowflake,
    pub(crate) max_video_channel_users: Option<i32>,
    pub(crate) approximate_member_count: Option<i32>,
    pub(crate) approximate_presence_count: Option<i32>,
}

#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq)]
pub enum VerificationLevel {
    None = 0,
    Low = 1,
    Medium = 2,
    High = 3,
    VeryHigh = 4,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum ExplicitFilterLevel {
    Disabled = 0,
    MembersWithoutRoles = 1,
    AllMembers = 2,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum MFALevel {
    Disabled = 0,
    Elevated = 1,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct GuildMember {
    pub(crate) user: User,
    pub(crate) nick: Option<String>,
    pub(crate) roles: Vec<Snowflake>,
    pub(crate) joined_at: Option<Timestamp>,
    pub(crate) premium_since: Option<String>,
    pub(crate) deaf: bool,
    pub(crate) mute: bool,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct AuditLog {
    pub(crate) webhooks: Vec<Webhook>,
    pub(crate) users: Vec<User>,
    pub(crate) audit_log_entries: Vec<AuditLogEntry>,
    pub(crate) integrations: Vec<Integration>,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Webhook {
    pub(crate) id: Snowflake,
    pub(crate) r#type: WebhookType,
    pub(crate) guild_id: Option<Snowflake>,
    pub(crate) channel_id: Snowflake,
    pub(crate) user: Option<User>,
    pub(crate) name: String,
    pub(crate) avatar: String,
    pub(crate) token: Option<String>,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum WebhookType {
    Incoming = 1,
    ChannelFollower = 2,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Integration {
    pub(crate) id: Snowflake,
    pub(crate) name: String,
    pub(crate) r#type: String,
    pub(crate) enabled: bool,
    pub(crate) syncing: bool,
    pub(crate) role_id: String,
    pub(crate) enable_emoticons: Option<String>,
    pub(crate) expire_behavior: IntegrationExpireBehavior,
    pub(crate) expire_grace_period: i32,
    pub(crate) user: User,
    pub(crate) account: IntegrationAccount,
    pub(crate) synced_at: Timestamp,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum IntegrationExpireBehavior {
    RemoveRole = 0,
    Kick = 1,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct IntegrationAccount {
    pub(crate) id: Snowflake,
    pub(crate) name: String,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct AuditLogEntry {
    pub(crate) target_id: Snowflake,
    pub(crate) changes: Option<Vec<AuditLogChange>>,
    pub(crate) user_id: Snowflake,
    pub(crate) id: Snowflake,
    pub(crate) action_type: AuditLogEvent,
    pub(crate) options: Option<OptionalAuditLogEntry>,
    pub(crate) reason: Option<String>,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum AuditLogEvent {
    GuildUpdate = 1,
    ChannelCreate = 10,
    ChannelUpdate = 11,
    ChannelDelete = 12,
    ChannelCreateOverwrite = 13,
    ChannelUpdateOverwrite = 14,
    ChannelDeleteOverwrite = 15,
    KickMember = 20,
    PruneMember = 21,
    AddMemberBan = 22,
    RemoveMemberBan = 23,
    UpdateMember = 24,
    UpdateMemberRole = 25,
    MoveMember = 26,
    DisconnectMember = 27,
    AddBot = 28,
    CreateRole = 30,
    UpdateRole = 31,
    DeleteRole = 32,
    CreateInvite = 40,
    UpdateInvite = 41,
    DeleteInvite = 42,
    CreateWebhook = 50,
    UpdateWebhook = 51,
    DeleteWebhook = 52,
    CreateEmoji = 60,
    UpdateEmoji = 61,
    DeleteEmoji = 62,
    DeleteMessage = 72,
    BulkDeleteMessage = 73,
    PinMessage = 74,
    UnpinMessage = 75,
    CreateIntegration = 80,
    UpdateIntegration = 81,
    DeleteIntegration = 82,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct OptionalAuditLogEntry {
    pub(crate) delete_member_days: String,
    pub(crate) members_removed: String,
    pub(crate) channel_id: Snowflake,
    pub(crate) message_id: Snowflake,
    pub(crate) count: String,
    pub(crate) id: Snowflake,
    pub(crate) r#type: String,
    pub(crate) role_name: String,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct AuditLogChange {
    pub(crate) new_value: Option<MixedType>,
    pub(crate) old_value: Option<MixedType>,
    pub(crate) key: String,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(untagged)]
pub enum MixedType {
    String(Option<String>),
    U8(Option<u8>),
    AuditLogEvent(Option<AuditLogEvent>),
    Int(Option<i32>),
    RoleVec(Option<Vec<Role>>),
    Bool(Option<bool>),
    OverwriteVec(Option<Vec<PermissionOverwrite>>),
}