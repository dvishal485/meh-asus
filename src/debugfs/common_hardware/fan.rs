use std::marker::PhantomData;

use crate::debugfs::{config::Hardware, config_trait::Config, error::HardwareError};

pub const DEV_ID: u64 = 0x110019;

pub const FAN: Hardware<FanMode> = Hardware {
    dev_id: DEV_ID,
    states_type: PhantomData,
    safe_read_mask: None,
};

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum FanMode {
    Standard = 0,
    Whispher = 1,
    Performace = 2,
    FullSpeed = 3,
}

impl TryFrom<u64> for FanMode {
    type Error = HardwareError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value as u8 {
            0 => Ok(FanMode::Standard),
            1 => Ok(FanMode::Whispher),
            2 => Ok(FanMode::Performace),
            3 => Ok(FanMode::FullSpeed),
            _ => Err(HardwareError::NotPossibleState { value }),
        }
    }
}

impl Config for FanMode {
    fn to_config(&self) -> String {
        (*self as u8).to_string()
    }
}
