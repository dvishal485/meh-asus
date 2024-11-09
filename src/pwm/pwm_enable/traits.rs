use crate::pwm::fan::AsusNbWmiFanMode;

use super::{
    base_path::BASE_PATH,
    error::{FanModeReadError, FanModeSetError, InputReadError, LabelReadError, PwmEnableError},
    PwmEnable, PwmEnableReadOnly, PwmEnableReadWrite,
};

/// ReadConfig trait is used to read the current configuration of the pwm device
///
/// These doesn't require write permission
pub trait ReadConfig {
    /// Get the current fan mode
    fn get_fan_mode(&self) -> Result<AsusNbWmiFanMode, FanModeReadError>;
    /// Get the label of the pwm device (example: `cpu_fan` or `gpu_fan`)
    fn get_label(&self) -> Result<String, LabelReadError>;
    /// Get the current input value (rpm of fan) of the pwm device
    fn get_input(&self) -> Result<u16, InputReadError>;
}

/// WriteConfig trait is used to write the configuration of the pwm device
///
/// These require write permission
pub trait WriteConfig {
    fn set_fan_mode(&mut self, mode: AsusNbWmiFanMode) -> Result<(), FanModeSetError>;
}

/// PwmHardware trait is used to interact with the pwm device
pub trait PwmHardware<T: PwmEnableState> {
    /// Create a new PwmEnable instance with the given path and pwm_id
    /// 
    /// Throws an error if the path and pwmid pair is not valid control file
    ///
    /// # Arguments
    ///
    /// * `path` - is the path to the pwm_enable device control file directory, 
    ///   it is generally not required to use this function directly, use
    ///   [find_and_get](PwmHardware::find_and_get) or [get](PwmHardware::get) functions instead
    ///
    /// * `pwm_id` is the pwm id of the pwm device (example: `0` for `pwm0_enable`)
    fn new(path: std::ffi::OsString, pwm_id: u8) -> Result<Self, PwmEnableError>
    where
        Self: Sized;

    /// Find the path of the pwm_enable device control file and create its PwmEnable instance
    /// 
    /// # Arguments
    /// 
    /// * `pwm_id` - PwmId of the pwm_enable device (example: `0` for `pwm0_enable`)
    fn find_and_get(pwm_id: u8) -> Result<Self, PwmEnableError>
    where
        Self: Sized,
    {
        let path = PwmEnable::<T>::find_hwmon_path(pwm_id)?;
        Self::new(path, pwm_id)
    }

    /// Get the PwmEnable instance with the given pwm id and hwmon id
    /// 
    /// # Arguments
    /// 
    /// * `pwm_id` - PwmId of the pwm_enable device (example: `0` for `pwm0_enable`)
    /// * `hwmon_id` - HwmonId of the pwm_enable device (example: `5` for `hwmon5`)
    /// 
    /// You may want to use [find_and_get](PwmHardware::find_and_get) instead of this function
    fn get(pwm_id: u8, hwmon_id: u8) -> Result<Self, PwmEnableError>
    where
        Self: PwmEnableState + Sized,
    {
        let path = format!("{BASE_PATH}/hwmon{hwmon_id}/pwm{pwm_id}_enable").into();
        Self::new(path, pwm_id)
    }

    /// Get the PWM ID of instance
    fn get_pwm_id(&self) -> u8;
    /// Get the file path of instance
    fn get_file_path(&self) -> &std::ffi::OsString;
    /// Get the file of instance
    fn get_file(&self) -> &std::fs::File;
}

pub trait PwmEnableState {
    #[doc(hidden)]
    /// Returns true if the pwm device has write permission
    fn write_permission() -> bool {
        false
    }
}

impl PwmEnableState for PwmEnableReadOnly {}
impl PwmEnableState for PwmEnableReadWrite {
    fn write_permission() -> bool {
        true
    }
}
