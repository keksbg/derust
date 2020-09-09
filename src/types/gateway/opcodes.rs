//! # OPCode statuses and Gateway close codes
//! These are the `enum`s that will be returned to you
//! on every single gateway event that is sent back.
//!
//! You can find additional information about each one
//! on the [Developer documentation for Discord](https://discord.dev/)
use serde::{Serialize, Deserialize};

/// These are the OPCodes used by both the client and
/// the gateway server to communicate.
///
/// They are found in the `op` header of each payload.
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum OPCode {
    Dispatch = 0, // y
    Heartbeat = 1, // n
    Identify = 2, // n
    PresenceUpdate = 3, // n
    VoiceStateUpdate = 4, // n
    Resume = 6, // n
    Reconnect = 7, // n
    RequestGuildMembers = 8, // n
    InvalidSession = 9, // n
    Hello = 10,
    HeartbeatACK = 11,
}

/// These are the close codes that will be sent
/// by the server when you get disconnected.
///
/// All of them should get automatically handled,
/// either by panicking and exiting with the
/// OPCode as the status, or by just attempting
/// to resume the session or just starting a new one.
#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum CloseCode {
    /// Unknown reason for closing. Reconnect.
    Unknown = 4000,
    /// We sent an incorrect OPCode. You should notify the library
    /// maintainers about this.
    UnknownOPCode = 4001,
    /// Failed to decode the payload. You should notify the library
    /// maintainers.
    DecodeError = 4002,
    /// Didn't send an Identify OPCode for you to do certain actions.
    ///
    /// This is possibly due to a network problem modifying the packet queue,
    /// or due to a library error.
    Unauthenticated = 4003,
    /// Failed to authenticate. This automatically means a wrong token.
    AuthFailed = 4004,
    /// You have already authenticated in this session. You should notify
    /// the library maintainers.
    AlreadyAuthenticated = 4005,
    /// Invalid sequence number on which to resume. Probably took too long.
    InvalidSeq = 4007,
    /// You have been ratelimited by the gateway for sending too many
    /// payloads. You will be disconnected and should start a *new* session
    Ratelimited = 4008,
    /// Session timed out because of you either not responding to requests
    /// or not sending heartbeats. Reconnect and start a new one.
    Timeout = 4009,
    /// Invalid shard on the `Identify` payload. Check your configuration.
    InvalidShard = 4010,
    /// Way too many guild objects will be sent for you to not shard,
    /// so you should shard. Reconnect with a new sharding scheme.
    ShardingRequired = 4011,
    /// Requested an invalid gateway version. Reconnect on the correct one.
    InvalidVersion = 4012,
    /// Sent an invalid intent string of values. Most-likely calculated
    /// incorrectly.
    InvalidIntent = 4013,
    /// You do not have access to this intent because:
    ///
    /// a. You haven't enabled it in the developer dashboard or;
    ///
    /// b. You aren't whitelisted for it.
    DisallowedIntent = 4014,
}