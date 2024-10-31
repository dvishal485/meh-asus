use crate::fan::AsusNbWmiFanMode;

use super::{
    error::{FanModeReadError, FanModeSetError, InputReadError, LabelReadError, PwmEnableError},
    pwm_enable::BASE_PATH,
    PwmEnable, PwmEnableReadOnly, PwmEnableReadWrite,
};

pub trait ReadConfig {
    fn get_fan_mode(&self) -> Result<AsusNbWmiFanMode, FanModeReadError>;
    fn get_label(&self) -> Result<String, LabelReadError>;
    fn get_input(&self) -> Result<u16, InputReadError>;
}

pub trait WriteConfig {
    fn set_fan_mode(&mut self, mode: AsusNbWmiFanMode) -> Result<(), FanModeSetError>;
}

pub trait PwmHardware {
    fn new(path: std::ffi::OsString, pwmid: u8) -> Result<Self, PwmEnableError>
    where
        Self: Sized;

    fn find_and_get(pwm_id: u8) -> Result<Self, PwmEnableError>
    where
        Self: PwmEnableState + Sized,
    {
        let path = PwmEnable::<Self>::find_hwmon_path(pwm_id)?;
        Self::new(path, pwm_id)
    }

    fn get(pwm_id: u8, hwmon_id: u8) -> Result<Self, PwmEnableError>
    where
        Self: PwmEnableState + Sized,
    {
        let path: std::ffi::OsString =
            format!("{BASE_PATH}/hwmon{hwmon_id}/pwm{pwm_id}_enable").into();
        // PwmEnable::new(path, pwm_id);
        todo!()
    }
}

pub trait PwmEnableState {
    fn write_permission() -> bool;
}

impl PwmEnableState for PwmEnableReadWrite {
    fn write_permission() -> bool {
        true
    }
}
impl PwmEnableState for PwmEnableReadOnly {
    fn write_permission() -> bool {
        false
    }
}
