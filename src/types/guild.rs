use super::user::User;
use serde::{Serialize, Deserialize};
use super::role::Role;
use super::message::Emoji;
use super::voice::VoiceState;
use super::channel::Channel;
use super::gateway::activity::PresenceUpdate;
use super::permission::PermissionOverwrite;
use crate::types::{Snowflake, Timestamp};
use super::CachedTypes;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Guild {
    pub id: Snowflake,
    pub name: String,
    pub icon: String,
    pub splash: String,
    pub discovery_splash: String,
    #[serde(default)]
    pub owner: bool,
    pub owner_id: Snowflake,
    pub permissions: Option<i32>, //todo: replace with proper permissions
    pub region: String,
    pub afk_channel_id: Snowflake,
    pub afk_timeout: i32,
    #[serde(default)]
    pub embed_enabled: bool,
    pub embed_channel_id: Option<Snowflake>,
    pub verification_level: VerificationLevel,
    pub default_message_notifications: i32,
    pub explicit_content_filter: ExplicitFilterLevel,
    pub roles: Vec<Role>,
    pub emojis: Vec<Emoji>,
    /// See https://discord.com/developers/docs/resources/guild#guild-object-guild-features
    pub features: Vec<String>,
    pub mfa_level: MFALevel,
    pub application_id: Snowflake,
    #[serde(default)]
    pub widget_enabled: bool,
    pub widget_channel_id: Option<Snowflake>,
    pub system_channel_id: Snowflake,
    pub system_channel_flags: i32,
    pub rules_channel_id: Snowflake,
    pub joined_at: Option<Timestamp>, // utc time
    #[serde(default)]
    pub large: bool,
    #[serde(default)]
    pub unavailable: bool,
    pub member_count: Option<i32>,
    pub voice_states: Option<Vec<VoiceState>>,
    pub members: Option<Vec<GuildMember>>,
    pub channels: Option<Vec<Channel>>,
    pub presences: Option<Vec<PresenceUpdate>>,
    pub max_presences: Option<i32>,
    pub max_members: Option<i32>,
    pub vanity_url_code: String,
    pub description: String,
    pub banner: String,
    pub premium_tier: i32,
    pub premium_subscription_count: Option<i32>,
    pub preferred_locale: String,
    pub public_updates_channel_id: Snowflake,
    pub max_video_channel_users: Option<i32>,
    pub approximate_member_count: Option<i32>,
    pub approximate_presence_count: Option<i32>,
}

impl CachedTypes for Guild {}

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
    pub user: User,
    pub nick: Option<String>,
    pub roles: Vec<Snowflake>,
    pub joined_at: Option<Timestamp>,
    pub premium_since: Option<String>,
    pub deaf: bool,
    pub mute: bool,
}

impl CachedTypes for GuildMember {}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct AuditLog {
    pub webhooks: Vec<Webhook>,
    pub users: Vec<User>,
    pub audit_log_entries: Vec<AuditLogEntry>,
    pub integrations: Vec<Integration>,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Webhook {
    pub id: Snowflake,
    pub r#type: WebhookType,
    pub guild_id: Option<Snowflake>,
    pub channel_id: Snowflake,
    pub user: Option<User>,
    pub name: String,
    pub avatar: String,
    pub token: Option<String>,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum WebhookType {
    Incoming = 1,
    ChannelFollower = 2,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Integration {
    pub id: Snowflake,
    pub name: String,
    pub r#type: String,
    pub enabled: bool,
    pub syncing: bool,
    pub role_id: String,
    pub enable_emoticons: Option<String>,
    pub expire_behavior: IntegrationExpireBehavior,
    pub expire_grace_period: i32,
    pub user: User,
    pub account: IntegrationAccount,
    pub synced_at: Timestamp,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum IntegrationExpireBehavior {
    RemoveRole = 0,
    Kick = 1,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct IntegrationAccount {
    pub id: Snowflake,
    pub name: String,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct AuditLogEntry {
    pub target_id: Snowflake,
    pub changes: Option<Vec<AuditLogChange>>,
    pub user_id: Snowflake,
    pub id: Snowflake,
    pub action_type: AuditLogEvent,
    pub options: Option<OptionalAuditLogEntry>,
    pub reason: Option<String>,
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
    pub delete_member_days: String,
    pub members_removed: String,
    pub channel_id: Snowflake,
    pub message_id: Snowflake,
    pub count: String,
    pub id: Snowflake,
    pub r#type: String,
    pub role_name: String,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct AuditLogChange {
    pub new_value: Option<MixedType>,
    pub old_value: Option<MixedType>,
    pub key: String,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(untagged)]
pub enum MixedType {
    String(Option<String>),
    U8(Option<u8>),
    AuditLogEvent(Option<AuditLogEvent>),
    Int(Option<i32>),
    RoleVec(Option<Vec<Role>>),
    Bool(bool),
    OverwriteVec(Option<Vec<PermissionOverwrite>>),
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)] // serialize required because of enum in gateway/payloads.rs
pub struct PartialGuild {
    pub id: Snowflake,
    pub unavailable: bool,
}