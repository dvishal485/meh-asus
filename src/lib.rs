//! Abstraction over ASUS hardware configurations to control it programatically on Linux.
//!
//! This can utilize any hardware's DEV_ID, to read and modify its configuration as defined by the user.
//!
//! You can find the DEV_ID of the hardware you're interested in
//! [ASUS WMI source code](https://github.com/torvalds/linux/blob/master/drivers/platform/x86/asus-wmi.c).
//!
//! ## Common Hardware
//!
//! Some of the hardware dev id and their states are defined by default, serving as an example as well
//! as direct abstraction over them.
//!
//! Users can refer to them to use it themselves, or to create abstraction over some other
//! hardware component of their choice. Please reference the "examples" directory.

#![cfg(target_os = "linux")]

#[cfg(feature = "pwm")]
pub mod debugfs;
#[cfg(feature = "pwm")]
pub mod pwm;

#[cfg(not(feature = "pwm"))]
mod debugfs;

pub use debugfs::*;
