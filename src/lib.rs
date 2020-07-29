#![feature(in_band_lifetimes)]
#[macro_use] extern crate serde;

pub mod types;
pub mod helpers;
static API_URL: &str = "https://discord.com/api/v6";
static GATEWAY_URL: &str = "wss://gateway.discord.gg";
