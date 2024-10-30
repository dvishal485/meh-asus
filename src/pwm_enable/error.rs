use super::AsusNbWmiFanMode;
use std::num::ParseIntError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PwmEnableError {
    #[error("Failed to set fan to `{value}`. Unsupported value.")]
    IllegalFanModeValue { value: AsusNbWmiFanMode },

    #[error("Hardware is incompatible for `pwm{pwm_id}_enable`")]
    UnsupportedHardware { pwm_id: u8 },

    #[error("Label couldn't be accessed! {error}")]
    LabelIncompatible { error: std::io::Error },

    #[error("Error occured while reading the label! {error}")]
    LabelReadError { error: std::io::Error },

    #[error("Input couldn't be interpreted as a number! {parse_error}")]
    NonNumericInputValue { parse_error: ParseIntError },
}
