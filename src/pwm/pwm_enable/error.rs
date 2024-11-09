use crate::pwm::fan::AsusNbWmiFanModeError;
use crate::pwm::fan::AsusNbWmiFanMode;
use std::num::ParseIntError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PwmEnableError {
    #[error("Hardware is incompatible for `pwm{pwm_id}_enable`")]
    UnsupportedHardware { pwm_id: u8 },

    #[error("Hardware config file couldn't be accessed `pwm{pwm_id}_enable`! {error}")]
    UnableToAccessHardware { pwm_id: u8 , error: std::io::Error},

    #[error("The action failed with System I/O error! {error}")]
    IOError { error: std::io::Error },
}

#[derive(Debug, Error)]
pub enum InputReadError {
    #[error("Input couldn't be interpreted as a number! {parse_error}")]
    NonNumericInputValue { parse_error: ParseIntError },

    #[error("Input couldn't be accessed! {error}")]
    InputIncompatible { error: std::io::Error },

    #[error("Error occured while reading the input! {error}")]
    IOReadError { error: std::io::Error },
}

#[derive(Debug, Error)]
pub enum FanModeReadError {
    #[error("{error}")]
    AsusNbWmiFanModeError { error: AsusNbWmiFanModeError },
    
    #[error("Error occured while reading the label! {error}")]
    IOReadError { error: std::io::Error },
}

#[derive(Debug, Error)]
pub enum LabelReadError {
    #[error("Label couldn't be accessed! {error}")]
    LabelIncompatible { error: std::io::Error },

    #[error("Error occured while reading the label! {error}")]
    IOReadError { error: std::io::Error },
}

#[derive(Debug, Error)]
pub enum FanModeSetError {
    #[error("Failed to set fan to `{value}`. Unsupported value.")]
    IllegalFanModeValue { value: AsusNbWmiFanMode },

    #[error("Requested fan mode couldn't be set! {error}")]
    UnknownError { error: std::io::Error },
    
    #[error("OS rejected fan mode switch. AC power maybe required to set the fan mode! {error}")]
    AcPowerRequired { error: std::io::Error },
}
