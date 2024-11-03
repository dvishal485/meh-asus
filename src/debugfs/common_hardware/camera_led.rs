use std::marker::PhantomData;

use crate::debugfs::{config::Hardware, config_trait::Config, error::HardwareError};

pub const DEV_ID: u64 = 0x60079;

pub const CAMERA_LED: Hardware<CameraLedState> = Hardware {
    dev_id: DEV_ID,
    states_type: PhantomData,
    safe_read_mask: None,
};

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum CameraLedState {
    Off = 0,
    On = 1,
}
impl TryFrom<u64> for CameraLedState {
    type Error = HardwareError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(CameraLedState::Off),
            1 => Ok(CameraLedState::On),
            _ => Err(HardwareError::NotPossibleState { value }),
        }
    }
}

impl Config for CameraLedState {
    fn to_config(&self) -> String {
        (*self as u8).to_string()
    }
}
