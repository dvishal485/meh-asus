use crate::debugfs::{Config, error::HardwareError};


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum LedState {
    Off = 0,
    On = 1,
}

impl TryFrom<u64> for LedState {
    type Error = HardwareError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(LedState::Off),
            1 => Ok(LedState::On),
            _ => Err(HardwareError::NotPossibleState { value }),
        }
    }
}

impl Config for LedState {
    fn to_config(&self) -> String {
        (*self as u8).to_string()
    }
}
