use super::{config_trait::Config, error::HardwareError};
use std::{fs, marker::PhantomData};

#[derive(Debug, Clone, Copy)]
pub struct Hardware<T>
where
    T: Config + Copy,
{
    pub(crate) dev_id: u64,
    pub(crate) states_type: PhantomData<T>,
}

macro_rules! path {
    ($x:expr) => {
        concat!("/sys/kernel/debug/asus-nb-wmi/", $x)
    };
}

impl<T> Hardware<T>
where
    T: Config + Copy,
{
    pub fn new(dev_id: u64) -> Self {
        Hardware {
            dev_id,
            states_type: PhantomData,
        }
    }

    fn open(&self) -> Result<(), HardwareError<T>> {
        fs::write(path!("dev_id"), self.dev_id.to_string())
            .map_err(|error| HardwareError::DevIdWriteFailed { error })?;

        Ok(())
    }

    pub fn apply(&self, ctrl_param: T) -> Result<(), HardwareError<T>> {
        self.open()?;

        fs::write(path!("ctrl_param"), ctrl_param.to_config())
            .map_err(|e| HardwareError::CtrlParamWriteFailed { error: e })?;

        fs::read(path!("devs")).map_err(|e| HardwareError::ConfigApplyFailed { error: e })?;

        Ok(())
    }

    pub fn read(&self) -> Result<Result<T, T>, HardwareError<T>> {
        self.open()?;

        let devs = fs::read_to_string(path!("devs"))
            .map_err(|e| HardwareError::ConfigApplyFailed { error: e })?;

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
                            let value = dbg!(value.strip_prefix("0x").or(Some(value)).unwrap());
                            u64::from_str_radix(value, 16).map_err(|e| {
                                HardwareError::InvalidHexadecimalValue {
                                    value: value.to_string(),
                                    error: e,
                                }
                            })
                        },
                    )
                })
            })
            .ok_or(HardwareError::UnexpectedConfigFormat {
                value: devs.to_string(),
                hardware: *self,
            })?;

        // optionally verify the dev_id, if it is present.
        // if no dev_id is inferred, we can only pray to god that it is the correct value.
        let value = value?;
        let value =
            T::try_from(value).map_err(|_| HardwareError::NotPossibleState { value: value })?;

        Ok(if let Some(inferred_dev_id) = inferred_dev_id {
            if inferred_dev_id != self.dev_id {
                return Err(HardwareError::UnexpectedConfigFormat {
                    value: devs.to_string(),
                    hardware: *self,
                });
            }
            Ok(value)
        } else {
            eprintln!("dev_id not found in the config file. This is unexpected. Cannot assert the correctness of the config value read.");
            Err(value)
        })
    }
}
