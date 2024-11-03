use std::num::ParseIntError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum HardwareError
{
    #[error("Failed to write dev_id! {error}")]
    DevIdWriteFailed { error: std::io::Error },

    #[error("Failed to write applied config! {error}")]
    CtrlParamWriteFailed { error: std::io::Error },

    #[error("Failed to apply the given config! {error}")]
    ConfigApplyFailed { error: std::io::Error },

    #[error("Cannot read the config due to unexpected format!\nExpected: `DEVS({}, {{some_value}}) = {{some_value}}\nFound: {value}", dev_id)]
    UnexpectedConfigFormat {
        value: String,
        dev_id: u64,
    },


    #[error("The given string `{value}` cannot be interpreted as hexadecimal value! {error}")]
    InvalidHexadecimalValue { value: String, error: ParseIntError },

    #[error("The state value `{value}` is not listed as a possible state for the hardware!")]
    NotPossibleState { value: u64 },

    #[error("Failed to read the currect config file! {error}")]
    DstsFileStateReadFailed { error: std::io::Error },


    #[error("Cannot read the config due to unexpected format!\nExpected: `DSTS({}) = {{some_value}}\nFound: {value}", dev_id)]
    UnexpectedConfigDstsFormat {
        value: String,
        dev_id: u64
    },

    #[error("Mask is not set. Cannot read the config without mask!\nYou might want to call `Hardware::read` first with mutable reference to self to set the mask.")]
    UsedWithoutMaskSet 
}
