//! Relay server in local and server side implementations.

#[cfg(feature = "trust-dns")]
pub(crate) mod dns_resolver;
mod loadbalancing;
pub mod local;
pub mod server;
pub mod socks5;
pub mod tcprelay;
pub mod udprelay;
pub(crate) mod utils;
