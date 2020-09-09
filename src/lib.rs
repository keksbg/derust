//! `derust`, an async-first library for the Discord API.
//!
//! derust aims to be a basic, async-first and very
//! customizable Discord API library for Rust.
//!
//! # Minimalist Example
//!
//! ```rust
//! use derust::types::gateway::builder::IdentifyObject;
//! use derust::types::gateway::client::Client;
//! use derust::types::gateway::cache::Caches;
//! use enumflags2::BitFlags;
//!
//! #[tokio::main]
//! async fn main() {
//! let io = IdentifyObject::new(String::from("token"))
//!         .intents(BitFlags::from_bits(0b110010).unwrap().bits()) // guilds, guild members, guild integrations
//!         .compress(true)
//!         .large_threshold(250)
//!         .guild_subscriptions(false);
//!
//!     let cache = Caches::initialize();
//!
//!     let ws = Client::new(io, cache)
//!         .login();
//!
//!     while let Some(e) = ws.recv().await {
//!         match e {
//!             // the events you want to catch here
//!             _ => {
//!                 println!("Didn't handle event!");
//!             }
//!         }
//!     }
//! }
//! ```
//!
//! # Events
//!
//! Events are handled *directly* by the user themselves,
//! mostly by using `match` on the events that are sent back
//! by the [`Client`] via a [`tokio::sync::mpsc`](https://docs.rs/tokio/0.2.22/tokio/sync/mpsc/fn.channel.html).
//!
//! [`Client`]: types::gateway::client::Client
//!
//! # Crate Features
//! * Default: `cache`
//!
//! * `cache`: Utilize the built-in cache with
//! the [`std::collections::HashMap`] type, or specify your own.
//!
//! # Caching
//! If you are just utilizing the default features, you will
//! be using an unlimited [`std::collections::HashMap`] to store the
//! cached elements.
//!
//! While it is theoretically fast as it is [`O(1)`](https://en.wikipedia.org/wiki/Big_O_notation)
//! to insert, get and remove elements, you might want to use a
//! separate service like Redis or your own type, for that you have to make
//! it implement the [`Cache`](types/gateway/builder/trait.Cache)
//! trait.
//!
//! If you would otherwise like to not cache *anything*, because you
//! could be running in a situation with a low amount of usable RAM,
//! you can just disable the default features of the crate, or disable
//! the caching of individual elements when constructing the [`Client`]
//! with the functions `TYPE_cache()` and specifying `None`.
//! ```rust
//! Client::new(
//!     IdentifyObject::new("token")
//! ).channel_cache(None).login()
//! ```
//!
//! [`Client`]: types::gateway::client::Client
#![feature(in_band_lifetimes)]
#![allow(dead_code)]
//#![warn(missing_docs)] not now, lol (warning: 263 warnings emitted)
#[macro_use] extern crate serde;

pub mod types;
mod helpers;
const API_URL: &str = "https://discord.com/api/v6";
const GATEWAY_URL: &str = "wss://gateway.discord.gg";
