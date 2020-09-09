use super::user::User;
use super::guild::GuildMember;
use super::channel::ChannelMention;
use serde::{Serialize, Deserialize};
use super::role::Role;
use crate::types::{Snowflake, Timestamp};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct MentionedUsers {
    pub id: Snowflake,
    pub username: String,
    pub discriminator: String,
    pub avatar: String,
    pub verified: bool,
    pub email: String,
    pub flags: i64,
    pub premium_type: i64,
    pub public_flags: i64,
    pub member: GuildMember,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Message {
    pub id: Snowflake,
    pub channel_id: Snowflake,
    pub guild_id: Option<Snowflake>,
    pub author: User,
    pub member: GuildMember,
    pub content: String,
    pub timestamp: Timestamp,
    pub edited_timestamp: Option<Timestamp>,
    pub tts: bool,
    pub mention_everyone: bool,
    pub mentions: Vec<MentionedUsers>,
    pub mention_roles: Vec<String>,
    pub mention_channels: Option<Vec<ChannelMention>>,
    pub attachments: Vec<Attachment>,
    pub embeds: Vec<Embed>,
    pub reactions: Vec<MessageReaction>,
    pub nonce: Option<String>,
    pub pinned: bool,
    pub webhook_id: Option<Snowflake>,
    pub r#type: MessageType,
    pub activity: Option<MessageActivity>,
    pub application: Option<MessageApplication>,
    pub message_reference: Option<MessageReference>,
    pub flags: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct MessageReaction {
    pub count: i32,
    pub me: bool,
    pub emoji: Emoji,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct MessageActivity {
    pub r#type: MessageActivityType,
    pub party_id: Option<Snowflake>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum MessageActivityType {
    Join = 1,
    Spectate = 2,
    Listen = 3,
    JoinRequest = 5
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum MessageType {
    Default = 0,
    AddRecipient = 1,
    RemoveRecipient = 2,
    Call = 3,
    ChannelNameChange = 4,
    ChannelIconChange = 5,
    ChannelPinnedMessage = 6,
    GuildMemberJoin = 7,
    UserBoost = 8,
    UserBoostTier1 = 9,
    UserBoostTier2 = 10,
    UserBoostTier3 = 11,
    AddChannelFollow = 12,
    GuildDiscoveryDisqualified = 14,
    GuildDiscoveryRequalified = 15,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct MessageApplication {
    pub id: Snowflake,
    pub cover_image: Option<String>,
    pub description: String,
    pub icon: Option<String>,
    pub name: String
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct MessageReference {
    pub message_id: Snowflake,
    pub channel_id: Snowflake,
    pub guild_id: Snowflake,
}

pub struct AllowedMentions {
    pub parse: Vec<String>,
    pub roles: Vec<Snowflake>,
    pub users: Vec<Snowflake>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Attachment {
    pub id: Snowflake,
    pub filename: String,
    pub size: i32,
    pub url: String,
    pub proxy_url: String,
    pub height: i32,
    pub width: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Embed {
    pub title: Option<String>,
    pub r#type: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub timestamp: Option<Timestamp>,
    pub color: Option<i32>,
    pub footer: Option<Vec<EmbedFooter>>,
    pub image: Option<EmbedImage>,
    pub thumbnail: Option<EmbedThumbnail>,
    pub video: Option<EmbedVideo>,
    pub provider: Option<EmbedProvider>,
    pub author: Option<EmbedAuthor>,
    pub fields: Vec<EmbedField>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct EmbedFooter {
    pub text: String,
    pub icon_url: Option<String>,
    pub proxy_icon_url: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct EmbedImage {
    pub url: Option<String>,
    pub proxy_url: Option<String>,
    pub height: Option<i32>,
    pub width: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct EmbedProvider {
    pub name: Option<String>,
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct EmbedAuthor {
    pub name: Option<String>,
    pub url: Option<String>,
    pub icon_url: Option<String>,
    pub proxy_icon_url: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct EmbedField {
    pub name: String,
    pub value: String,
    #[serde(default)]
    pub inline: bool
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct EmbedThumbnail {
    pub url: Option<String>,
    pub proxy_url: Option<String>,
    pub height: Option<i32>,
    pub width: Option<i32>
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct EmbedVideo {
    pub url: Option<String>,
    pub height: Option<i32>,
    pub width: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Emoji {
    pub id: Snowflake,
    pub name: String,
    pub roles: Option<Role>,
    pub user: Option<User>,
    #[serde(default)]
    pub require_colons: bool,
    #[serde(default)]
    pub managed: bool,
    #[serde(default)]
    pub animated: bool,
    #[serde(default)]
    pub available: bool,
}