mod config;
pub use config::Hardware;
mod config_trait;
pub mod error;
pub use config_trait::Config;

#[cfg(feature = "common-hardware")]
pub mod common_hardware;