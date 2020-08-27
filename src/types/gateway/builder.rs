use serde::Serialize;
use crate::types::gateway::activity::{StatusUpdate};
use enumflags2::BitFlags;

#[derive(Serialize, Deserialize, Eq, PartialEq)]
pub struct IdentifyObject {
    token: String,
    properties: IdentifyProperties,
    compress: bool,
    large_threshold: Option<u8>,
    shard: Option<Vec<u8>>,
    presence: Option<StatusUpdate>,
    guild_subscriptions: Option<bool>,
    intents: Option<u32>,
    #[serde(skip)]
    auto_reidentify: bool,
}

#[derive(Serialize, Deserialize, Eq, PartialEq)]
pub struct IdentifyProperties {
    #[serde(rename = "$os")]
    os: String,
    #[serde(rename = "$browser")]
    browser: String,
    #[serde(rename = "$device")]
    device: String,
}

impl IdentifyObject {
    /// Construct a new `IdentifyObject` with the default values
    /// which are specified in the [Discord developer documentation](https://discord.dev/),
    /// the only exception being that we enable compression,
    /// and the specified token.
    pub fn new(token: String) -> Self {
        Self {
            token,
            properties: IdentifyProperties{
                os: String::from("Linux"),
                browser: String::from("derust"),
                device: String::from("derust"),
            },
            compress: true,
            large_threshold: None,
            shard: None,
            presence: None,
            guild_subscriptions: None,
            intents: None,
            auto_reidentify: false,
        }
    }
    /// Whether or not to use zlib compression (achieved with
    /// the [`miniz_oxide`](https://docs.rs/miniz_oxide/) crate)
    ///
    /// Default: true
    ///
    /// Note: This only impacts the server's ability to
    /// compress, we are only decompressing and sending
    /// standard uncompressed JSON to it.
    pub fn compress(&mut self, choice: bool) -> &mut Self {
        self.compress = choice;
        self
    }

    /// The gateway intents that will be used throughout this session.
    /// To construct them, use the [`Intents`](#Intents) enum as follows:
    /// ```rust
    /// use derust::types::gateway::builder::{Intents, IdentifyObject};
    /// use enumflags2::BitFlags;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut intents = BitFlags::from_flag(Intents::GuildMessages);
    ///     intents.insert(Intents::DirectMessages);
    ///     let mut identify = IdentifyObject::new(String::from("token here"))
    ///         .intents(intents.bits());
    /// }
    /// ```
    /// Or it is also possible to do:
    /// ```rust
    /// use derust::types::gateway::builder::{Intents, IdentifyObject};
    /// use enumflags2::BitFlags;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut intents = BitFlags::from_bits(0b010010); // GuildMembers and GuildIntegrations
    ///     let mut identify = IdentifyObject::new(String::from("token here"))
    ///         .intents(intents.bits().try_into().unwrap()); // you should probably never unwrap this, instead use something like the ? shorthand
    /// }
    /// ```
    ///
    /// Default: none
    pub fn intents(&mut self, intents: u32) -> &mut Self {
        self.intents = Some(intents);
        self
    }

    /// The presence that will be sent on the gateway IDENTIFY OPCode,
    /// which will be displayed until modified manually later on in code.
    ///
    /// To construct it, use the [`StatusUpdate::new`] builder.
    ///
    /// Default: none
    pub fn presence(&mut self, presence: StatusUpdate) -> &mut Self {
        self.presence = Some(presence);
        self
    }

    /// The large threshold is the threshold at which Discord will
    /// stop sending offline user objects in guilds with a greater
    /// or equal value to the one specified.
    ///
    /// Must be between 50 and 250, otherwise the default of 50
    /// will be used.
    pub fn large_threshold(&mut self, threshold: u8) -> &mut Self {
        if threshold >= 50 && threshold <= 250 {
            self.large_threshold = Some(threshold);
            self
        } else {
            self
        }
    }

    /// Whether or not to receive [`GuildMember`](../../guild/index.html) presence and typing
    /// events. Generally recommended to be set to false for larger bots,
    /// but it is set to true by default.
    pub fn guild_subscriptions(&mut self, choice: bool) -> &mut Self {
        self.guild_subscriptions = Some(choice);
        self
    }

    /// Whether or not to automatically re-identify once the session has been attempted to be
    /// resumed, but it failed and we were requested to re-identify completely.
    ///
    /// Default: `false`
    pub fn auto_reidentify(&mut self, choice: bool) -> &mut Self {
        self.auto_reidentify = choice;
        self
    }

}

/// For a definition of all the enum values, see
/// the [developer documentation](https://discord.com/developers/docs/topics/gateway#gateway-intents).
///
/// Pay careful attention to `GuildPresences` and
/// `GuildMembers` because they are **privileged** intents.
#[derive(Serialize, BitFlags, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum Intents {
    Guilds = 1 << 0,
    GuildMembers = 1 << 1,
    GuildBans = 1 << 2,
    GuildEmojis = 1 << 3,
    GuildIntegrations = 1 << 4,
    GuildWebhooks = 1 << 5,
    GuildInvites = 1 << 6,
    GuildVoiceStates = 1 << 7,
    GuildPresences = 1 << 8,
    GuildMessages = 1 << 9,
    GuildMessageReactions = 1 << 10,
    GuildMessageTyping = 1 << 11,
    DirectMessages = 1 << 12,
    DirectMessageReactions = 1 << 13,
    DirectMessageTyping = 1 << 14,
}