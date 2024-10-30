// based on https://wiki.archlinux.org/title/Fan_speed_control
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AsusNbWmiFanModeError {
    #[error("Unsupported fan mode `{}`, can only be `0`, `1`, or `2`. Refer https://wiki.archlinux.org/title/Fan_speed_control", value)]
    InvalidFanMode { value: u8 },

    #[error("Invalid numeric byte `{}`. Refer https://wiki.archlinux.org/title/Fan_speed_control", value)]
    NonNumericByte { value: u8 },
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AsusNbWmiFanMode {
    FullSpeed = b'0',
    Manual = b'1',
    Auto = b'2',
}

impl std::fmt::Display for AsusNbWmiFanMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}",self))
    }
}

impl TryFrom<u8> for AsusNbWmiFanMode {
    type Error = AsusNbWmiFanModeError;

    fn try_from(value: u8) -> Result<AsusNbWmiFanMode, Self::Error> {
        if value < b'0' || value > b'9' {
            Err(AsusNbWmiFanModeError::NonNumericByte { value })
        } else {
            match value {
                b'0' => Ok(AsusNbWmiFanMode::FullSpeed),
                b'1' => Ok(AsusNbWmiFanMode::Manual),
                b'2' => Ok(AsusNbWmiFanMode::Auto),
                _ => Err(AsusNbWmiFanModeError::InvalidFanMode { value }),
            }
        }
    }
}
