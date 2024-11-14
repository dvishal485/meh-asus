#![doc(hidden)]
use crate::debugfs::{error::StateError, Config};

/// Represents the state of LED Key.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum LedState {
    Off = 0,
    On = 1,
}

impl TryFrom<u64> for LedState {
    type Error = StateError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(LedState::Off),
            1 => Ok(LedState::On),
            _ => Err(StateError::NotPossibleState { value }),
        }
    }
}

impl Config for LedState {
    fn to_config(&self) -> String {
        (*self as u8).to_string()
    }
}
