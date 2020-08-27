//! # Gateway types
//! All the children `mod`s of this parent `mod`
//! will contain types that pertain to the gateway.
//!
//! Additionally, the [builder](builder/index.html) `mod`
//! will contain functions to build structs that are required
//! to initialize the connection to the gateway.
pub mod builder;
pub mod payloads;
pub mod opcodes;
pub mod activity;
pub mod client;
#[cfg(feature = "cache")]
pub mod cache;