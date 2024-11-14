//! Error types and messages for the debugfs module.

use std::num::ParseIntError;
use thiserror::Error;

/// A general error type for the hardware module.
///
/// All the errors in debugfs is mapped to one of these errors.
///
/// Every function will only throw a subset of these errors, and errors
/// have been logically separated into different types.
#[derive(Debug, Error)]
pub enum HardwareError {
    #[error("Dev ID setup error: {0}")]
    DevIdFileError(#[from] DevIdFileError),

    #[error("Control Param Error: {0}")]
    CtrlParamError(#[from] CtrlParamError),

    #[error("Config Apply Error: {0}")]
    ConfigApplyError(#[from] ConfigApplyError),

    #[error("Devs Config File Error: {0}")]
    DevsConfigFileError(#[from] DevsConfigFileError),

    #[error("Dsts Config File Error: {0}")]
    DstsConfigFileError(#[from] DstsConfigFileError),
    
    #[error("State Error: {0}")]
    StateError(#[from] StateError),
}

#[derive(Debug, Error)]
pub enum DevIdFileError {
    #[error("Failed to write dev_id! {error}")]
    WriteFailed { error: std::io::Error },
}

#[derive(Debug, Error)]
pub enum CtrlParamError {
    #[error("Failed to write applied config! {error}")]
    WriteFailed { error: std::io::Error },
}

#[derive(Debug, Error)]
pub enum ConfigApplyError {
    #[error("Failed to apply the given config! {error}")]
    ConfigApplyFailed { error: std::io::Error },
}

#[derive(Debug, Error)]
pub enum DevsConfigFileError {
    #[error("Cannot read the config due to unexpected format!\nExpected: `DEVS({}, {{some_value}}) = {{some_value}}\nFound: {value}", dev_id)]
    UnexpectedConfigFormat { value: String, dev_id: u64 },

    #[error("The given string `{value}` cannot be interpreted as hexadecimal value! {error}")]
    InvalidHexadecimalValue { value: String, error: ParseIntError },
}

#[derive(Debug, Error)]
pub enum StateError {
    #[error("The state value `{value}` is not listed as a possible state for the hardware!\nPlease initialize the hardware with set of correct possible states, without this type safety cannot be guaranteed.")]
    NotPossibleState { value: u64 },
}


#[derive(Debug, Error)]
pub enum DstsConfigFileError {
    #[error("Cannot read the config due to unexpected format!\nExpected: `DEVS({}, {{some_value}}) = {{some_value}}\nFound: {value}", dev_id)]
    UnexpectedConfigFormat { value: String, dev_id: u64 },

    #[error("The given string `{value}` cannot be interpreted as hexadecimal value! {error}")]
    InvalidHexadecimalValue { value: String, error: ParseIntError },

    #[error("Failed to read the currect config file! {error}")]
    StateReadFailed { error: std::io::Error },
}