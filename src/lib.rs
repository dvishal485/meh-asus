#[cfg(feature = "pwm")]
pub mod debugfs;
#[cfg(feature = "pwm")]
pub mod pwm;

#[cfg(not(feature = "pwm"))]
mod debugfs;
#[cfg(not(feature = "pwm"))]
pub use debugfs::*;
