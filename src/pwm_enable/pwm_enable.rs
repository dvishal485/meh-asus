use super::error::PwmEnableError;
use crate::fan::AsusNbWmiFanMode;
use anyhow::Result;
use state_shift::{states, type_state};
use std::{ffi::OsString, fs::File, io::Read, os::unix::fs::FileExt, path::Path};

const BASE_PATH: &str = "/sys/devices/platform/asus-nb-wmi/hwmon/";

pub struct PwmEnableAbstraction {}

#[type_state(state_slots = 1, default_state = ReadOnly)]
pub struct PwmEnable {
    pub(crate) file: File,
    pub(crate) path: OsString,
    pub(crate) pwmid: u8,
}

impl PwmEnableAbstraction {
    pub fn get_read_only(pwm_id: u8, hwmon_id: u8) -> Result<PwmEnable<PwmEnableReadOnly>> {
        let path = format!("{BASE_PATH}/hwmon{hwmon_id}/pwm{pwm_id}_enable").into();
        PwmEnable::new(path, pwm_id)
    }

    pub fn get_read_write(pwm_id: u8, hwmon_id: u8) -> Result<PwmEnable<PwmEnableReadWrite>> {
        let path = format!("{BASE_PATH}/hwmon{hwmon_id}/pwm{pwm_id}_enable").into();
        PwmEnable::mut_new(path, pwm_id)
    }

    pub fn find_hwmon_path(pwm_id: u8) -> Result<OsString, PwmEnableError> {
        let path = Path::new(BASE_PATH.into());

        (0..=9_u8)
            .map(|hwmon_id| path.join(format!("hwmon{hwmon_id}/pwm{pwm_id}_enable")))
            // .inspect(|f| eprintln!("Looking for {f:?}"))
            .find(|path| (path.try_exists()).ok().is_some_and(|s| s))
            .map(|s| s.into_os_string())
            .ok_or(PwmEnableError::UnsupportedHardware { pwm_id })
    }

    pub fn find_and_get_read_only(pwm_id: u8) -> Result<PwmEnable<PwmEnableReadOnly>> {
        let path = Self::find_hwmon_path(pwm_id)?;
        PwmEnable::new(path, pwm_id)
    }

    pub fn find_and_get_read_write(pwm_id: u8) -> Result<PwmEnable<PwmEnableReadWrite>> {
        let path = Self::find_hwmon_path(pwm_id)?;
        PwmEnable::mut_new(path, pwm_id)
    }
}

#[states(ReadOnly, ReadWrite)]
impl PwmEnable {
    #[require(ReadOnly)]
    #[switch_to(ReadOnly)]
    pub fn new(path: OsString, pwmid: u8) -> Result<PwmEnable> {
        let file = File::options()
            .read(true)
            .write(false)
            .open(Path::new(&path))?;
        Ok(PwmEnable { file, path, pwmid })
    }

    #[require(ReadWrite)]
    #[switch_to(ReadWrite)]
    pub fn mut_new(path: OsString, pwmid: u8) -> Result<PwmEnable> {
        let file = File::options()
            .read(true)
            .write(true)
            .open(Path::new(&path))?;
        Ok(PwmEnable { file, path, pwmid })
    }

    #[require(A)]
    pub fn get_fan_mode(&self) -> Result<AsusNbWmiFanMode> {
        let mut profile: [u8; 1] = [0];
        self.file.read_exact_at(&mut profile, 0)?;

        let fanmode = AsusNbWmiFanMode::try_from(profile[0])?;
        Ok(fanmode)
    }

    #[require(ReadWrite)]
    pub fn set_fan_mode(&mut self, mode: AsusNbWmiFanMode) -> Result<()> {
        let Err(e) = self.file.write_all_at(&[mode as u8], 0) else {
            return Ok(());
        };

        match e.raw_os_error() {
            Some(22) => anyhow::bail!(PwmEnableError::IllegalFanModeValue { value: mode }),
            _ => anyhow::bail!("Failed to set fan mode. {e}"),
        }
    }

    #[require(ReadOnly)]
    #[switch_to(ReadWrite)]
    pub fn make_writable(self) -> Result<PwmEnable> {
        let file = File::options().read(true).write(true).open(&self.path)?;
        Ok(PwmEnable {
            file,
            path: self.path,
            pwmid: self.pwmid,
        })
    }

    #[require(A)]
    pub fn get_label(&self) -> Result<String> {
        let path = Path::new(&self.path);
        let label_path = path.with_file_name(format!("fan{}_label", self.pwmid));

        let mut file = File::options()
            .read(true)
            .write(false)
            .open(label_path)
            .map_err(|e| PwmEnableError::LabelIncompatible { error: e })?;

        let mut buf = String::new();
        file.read_to_string(&mut buf)?;

        if buf.ends_with('\n') {
            buf.pop();
        }

        Ok(buf)
    }

    #[require(A)]
    pub fn get_input(&self) -> Result<u16> {
        let path = Path::new(&self.path);
        let label_path = path.with_file_name(format!("fan{}_input", self.pwmid));

        let mut file = File::options()
            .read(true)
            .write(false)
            .open(label_path)
            .map_err(|e| PwmEnableError::LabelIncompatible { error: e })?;

        let mut buf = String::new();
        file.read_to_string(&mut buf)?;

        if buf.ends_with('\n') {
            buf.pop();
        }
        let input = buf
            .parse::<u16>()
            .map_err(|e| (PwmEnableError::NonNumericInputValue { parse_error: e }))?;

        Ok(input)
    }
}
