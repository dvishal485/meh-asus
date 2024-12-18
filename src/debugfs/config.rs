//! Hardware abstraction to control the hardware configurations.

use super::{error::*, Config};
use std::{cell::Cell, fs, marker::PhantomData};

/// Provides a safe interface to control the hardware configurations
/// initialized with the valid state configuration enum of the hardware.
#[derive(Debug, Clone)]
pub struct Hardware<State>
where
    State: Config,
{
    pub(crate) dev_id: u64,
    pub(crate) states_type: PhantomData<State>,
    pub(crate) safe_read_mask: Cell<Option<u64>>,
}

macro_rules! path {
    ($x:expr) => {
        concat!("/sys/kernel/debug/asus-nb-wmi/", $x)
    };
}

impl<State> Hardware<State>
where
    State: Config,
{
    /// Create a new Hardware instance with the given `dev_id`.
    ///
    /// It is just a wrapper to store value and its corresponding
    /// allowed states. Doesn't open the hardware config files.
    pub const fn new(dev_id: u64) -> Self {
        Hardware {
            dev_id,
            states_type: PhantomData,
            safe_read_mask: Cell::new(None),
        }
    }

    /// Open the hardware config files.
    ///
    /// Used for making affect to any changes to the hardware by reading the hardware file.
    fn open(&self) -> Result<(), DevIdFileError> {
        fs::write(path!("dev_id"), self.dev_id.to_string())
            .map_err(|error| DevIdFileError::WriteFailed { error })?;

        Ok(())
    }

    /// Applies the given state to the hardware.
    ///
    /// Refer [apply_any](Hardware::apply_any) to apply a state
    /// not in declared in configuration's state enum.
    pub fn apply(&self, ctrl_param: State) -> Result<(), HardwareError> {
        self._apply_raw(ctrl_param)
    }

    /// Applies any arbitary given config to the hardware.
    ///
    /// **Usecase:** Directly write a u64 value to the config file.
    ///
    /// Refer [apply](Hardware::apply) to apply a state declared in
    /// configuration's state enum, hence ensuring type safety.
    ///
    /// # Safety
    ///
    /// Not really unsafe, but we can have a "safer" alternative
    /// with [apply using declared state enum](Hardware::apply),
    /// marking it a valid state ensuring type safety.
    ///
    /// So marking this as unsafe to demote its usage.
    ///
    /// If still using this method, ensure that the value you are
    /// writing is valid for the hardware.
    pub unsafe fn apply_any(&self, ctrl_param: impl Config) -> Result<(), HardwareError> {
        self._apply_raw(ctrl_param)
    }

    /// Internal function serving as the base code of [apply](Hardware::apply) and [apply_any](Hardware::apply_any).
    /// The function is safe to use depending upon its usage, as the configuration application operation doesn't usually fail even if invalid.
    ///
    /// Use [apply](Hardware::apply) with a well defined state enum to ensure safety.
    fn _apply_raw(&self, ctrl_param: impl Config) -> Result<(), HardwareError> {
        self.open()?;

        fs::write(path!("ctrl_param"), ctrl_param.to_config())
            .map_err(|e| CtrlParamError::WriteFailed { error: e })?;

        fs::read(path!("devs")).map_err(|error| ConfigApplyError::ConfigApplyFailed { error })?;

        Ok(())
    }

    /// Read the current state of the hardware. **(Reliable)**
    ///
    /// Reads the currect state by overwriting config file to understand the current state.
    /// Hence it effectively reads, writes and resets again the DSTS file to determine the state.
    ///
    /// The mask required to read the state is generated on first run,
    /// though can be mapped using data given in the
    /// [asus-wmi driver code](https://github.com/torvalds/linux/blob/3e5e6c9900c3d71895e8bdeacfb579462e98eba1/include/linux/platform_data/x86/asus-wmi.h#L150-L158).
    ///
    /// Relates to [read_stale](Hardware::read_stale) function which is not reliable.
    pub fn read(&self) -> Result<State, HardwareError> {
        let current_raw_state = self.read_dsts()?;

        let mask = if let Some(mask) = self.safe_read_mask.get() {
            mask
        } else {
            unsafe { self.apply_any(0_u64) }?;
            let mask = self.read_dsts()?;
            self.safe_read_mask.set(Some(mask));

            // revert back to the original state
            unsafe { self.apply_any(current_raw_state ^ mask) }?;

            mask
        };

        let current_state_u8 = current_raw_state ^ mask;

        State::try_from(current_state_u8).map_err(|_| {
            StateError::NotPossibleState {
                value: current_state_u8,
            }
            .into()
        })
    }

    /// Read the raw value of the hardware config. This value is the actual state of the hardware,
    /// but cannot be directly mapped to the State enum without obtaining the default mask.
    ///
    /// Applies a basic check to ensure the config read is for the expected hardware dev_id.
    ///
    /// Refer to [read](Hardware::read) which uses [this method](Hardware::read_dsts) to read and map to the state.
    ///
    /// **Usecase:** If you want to read the raw value of the hardware config, and then map it to the state yourself.
    pub fn read_dsts(&self) -> Result<u64, HardwareError> {
        self.open()?;

        let config = fs::read_to_string(path!("dsts"))
            .map_err(|e| DstsConfigFileError::StateReadFailed { error: e })?;

        let (inferred_dev_id, value) = config
            .split_once('=')
            .ok_or_else(|| DstsConfigFileError::UnexpectedConfigFormat {
                value: config.to_owned(),
                dev_id: self.dev_id,
            })
            .map(|(dev_id_part, value_part)| {
                (
                    {
                        dev_id_part
                            .split_once(')')
                            .and_then(|(dev_id_part, _)| dev_id_part.split_once('('))
                            .and_then(|(_, dev_id)| {
                                dev_id
                                    .trim()
                                    .strip_prefix("0x")
                                    .and_then(|d| u64::from_str_radix(d, 16).ok())
                            })
                    },
                    {
                        let value_part = value_part.trim();
                        let value_part = value_part.strip_prefix("0x").unwrap_or(value_part);
                        u64::from_str_radix(value_part, 16).map_err(|e| {
                            DstsConfigFileError::InvalidHexadecimalValue {
                                value: value_part.to_string(),
                                error: e,
                            }
                            .into()
                        })
                    },
                )
            })?;

        if let Some(inferred_dev_id) = inferred_dev_id {
            if inferred_dev_id != self.dev_id {
                return Err(DstsConfigFileError::UnexpectedConfigFormat {
                    value: config,
                    dev_id: self.dev_id,
                }
                .into());
            }
        }

        value
    }

    /// Read the current state of the hardware. **(NOT RELIABALE)**
    ///
    /// Use [read](Hardware::read) instead.
    ///
    /// This function is not reliable, as it reports any value that is present in the config file.
    /// It may not be the actual state of the hardware.
    ///
    /// If you set the value of state yourself, and then read the state it will report the correct
    /// value. Unless the hardware changes the value itself or some other hardware changes the value.
    ///
    /// **Example:** Turn on camera_led, then read the state, it will report the correct value.
    /// Now switch fan to performance mode, and read the camera_led state, it will report the fan
    /// state instead, which may or may not even correspond to a valid state of camera_led. Even
    /// if the camera_led state is valid, it is not the actual state of the camera_led.
    ///
    /// [DSTS can be used to read the currect state accurately.](https://github.com/torvalds/linux/blob/3e5e6c9900c3d71895e8bdeacfb579462e98eba1/include/linux/platform_data/x86/asus-wmi.h#L150-L158)
    pub fn read_stale(&self) -> Result<Result<State, State>, HardwareError> {
        self.open()?;

        let devs = fs::read_to_string(path!("devs"))
            .map_err(|e| ConfigApplyError::ConfigApplyFailed { error: e })?;

        let (inferred_dev_id, value) = devs
            .trim_end()
            .strip_prefix("DEVS(")
            .and_then(|s| s.rsplit_once('='))
            .and_then(|(dev_id_part, _)| {
                dev_id_part.split_once(',').map(|(d, value)| {
                    (
                        d.trim()
                            .strip_prefix("0x")
                            .and_then(|d| u64::from_str_radix(d, 16).ok()),
                        {
                            let value = value.trim().strip_suffix(')').unwrap();
                            let value = value.strip_prefix("0x").unwrap_or(value);
                            u64::from_str_radix(value, 16).map_err(|e| {
                                DevsConfigFileError::InvalidHexadecimalValue {
                                    value: value.to_string(),
                                    error: e,
                                }
                            })
                        },
                    )
                })
            })
            .ok_or(DevsConfigFileError::UnexpectedConfigFormat {
                value: devs.to_string(),
                dev_id: self.dev_id,
            })?;

        // optionally verify the dev_id, if it is present.
        // if no dev_id is inferred, we can only pray to god that it is the correct value.
        let original_read_value = value?;
        let value =
            State::try_from(original_read_value).map_err(|_| StateError::NotPossibleState {
                value: original_read_value,
            })?;

        Ok(if let Some(inferred_dev_id) = inferred_dev_id {
            if inferred_dev_id != self.dev_id {
                return Err(DevsConfigFileError::UnexpectedConfigFormat {
                    value: devs.to_string(),
                    dev_id: self.dev_id,
                }
                .into());
            }
            Ok(value)
        } else {
            eprintln!("dev_id not found in the config file. This is unexpected. Cannot assert the correctness of the config value read.");
            Err(value)
        })
    }
}
