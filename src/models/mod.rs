
pub mod client;
pub mod routing;

#[cfg(feature = "lol")]
pub mod league;

#[cfg(feature = "clash")]
pub mod clash;

#[cfg(feature = "tournament")]
pub mod tournament;

pub mod account;
pub mod active_shard;

pub use client::*;