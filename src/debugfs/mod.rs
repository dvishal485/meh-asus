//! A general abstraction to read and modify configuration of any ASUS device hardware component
//! given their DEV_ID. DEV_ID of each hardware can be found in the source code of the ASUS WMI driver.
//! 
//! [ASUS WMI source code](https://github.com/torvalds/linux/blob/master/drivers/platform/x86/asus-wmi.c).


mod config;
pub use config::Hardware;
mod config_trait;
pub mod error;
pub use config_trait::Config;

#[cfg(feature = "common-hardware")]
pub mod common_hardware;