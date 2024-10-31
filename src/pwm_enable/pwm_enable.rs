use super::{
    base_path::BASE_PATH,
    error::{FanModeReadError, FanModeSetError, InputReadError, LabelReadError, PwmEnableError},
    traits::{PwmEnableState, PwmHardware, ReadConfig, WriteConfig},
};
use crate::fan::AsusNbWmiFanMode;
use std::{
    ffi::OsString, fs::File, io::Read, marker::PhantomData, os::unix::fs::FileExt, path::Path,
};

pub struct PwmEnableReadWrite;
pub struct PwmEnableReadOnly;

pub struct PwmEnable<T: PwmEnableState> {
    pub(crate) file: File,
    pub(crate) path: OsString,
    pub(crate) pwmid: u8,
    pub(crate) _state: PhantomData<T>,
}

impl<T: PwmEnableState> PwmHardware<T> for PwmEnable<T> {
    fn new(path: OsString, pwmid: u8) -> Result<Self, PwmEnableError> {
        let file = File::options()
            .read(true)
            .write(T::write_permission())
            .open(Path::new(&path))
            .map_err(|e| PwmEnableError::UnableToAccessHardware {
                pwm_id: pwmid,
                error: e,
            })?;
        Ok(PwmEnable {
            file,
            path,
            pwmid,
            _state: PhantomData,
        })
    }
    
    fn get_pwm_id(&self) -> u8 {
        self.pwmid
    }
    
    fn get_file_path(&self) -> &OsString {
        &self.path
    }
    
    fn get_file(&self) -> &File {
        &self.file
    }
    
}

impl<T: PwmEnableState> PwmEnable<T> {
    pub fn find_hwmon_path(pwm_id: u8) -> Result<OsString, PwmEnableError> {
        let path = Path::new(BASE_PATH.into());

        (0..=9_u8)
            .map(|hwmon_id| path.join(format!("hwmon{hwmon_id}/pwm{pwm_id}_enable")))
            // .inspect(|f| eprintln!("Looking for {f:?}"))
            .find(|path| (path.try_exists()).ok().is_some_and(|s| s))
            .map(|s| s.into_os_string())
            .ok_or(PwmEnableError::UnsupportedHardware { pwm_id })
    }
}

impl<T: PwmEnableState> ReadConfig for PwmEnable<T> {
    fn get_fan_mode(&self) -> Result<AsusNbWmiFanMode, FanModeReadError> {
        let mut profile: [u8; 1] = [0];
        self.file
            .read_exact_at(&mut profile, 0)
            .map_err(|e| FanModeReadError::IOReadError { error: e })?;

        AsusNbWmiFanMode::try_from(profile[0])
            .map_err(|e| FanModeReadError::AsusNbWmiFanModeError { error: e })
    }

    fn get_label(&self) -> Result<String, LabelReadError> {
        let path = Path::new(&self.path);
        let label_path = path.with_file_name(format!("fan{}_label", self.pwmid));

        let mut file = File::options()
            .read(true)
            .write(false)
            .open(label_path)
            .map_err(|e| LabelReadError::LabelIncompatible { error: e })?;

        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .map_err(|e| LabelReadError::IOReadError { error: e })?;

        if buf.ends_with('\n') {
            buf.pop();
        }

        Ok(buf)
    }

    fn get_input(&self) -> Result<u16, InputReadError> {
        let path = Path::new(&self.path);
        let input_path = path.with_file_name(format!("fan{}_input", self.pwmid));

        let mut file = File::options()
            .read(true)
            .write(false)
            .open(input_path)
            .map_err(|e| InputReadError::InputIncompatible { error: e })?;

        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .map_err(|e| InputReadError::IOReadError { error: e })?;

        if buf.ends_with('\n') {
            buf.pop();
        }
        let input = buf
            .parse::<u16>()
            .map_err(|e| (InputReadError::NonNumericInputValue { parse_error: e }))?;

        Ok(input)
    }
}

impl PwmEnable<PwmEnableReadOnly> {
    pub fn make_writable(self) -> Result<PwmEnable<PwmEnableReadWrite>, PwmEnableError> {
        let file = File::options()
            .read(true)
            .write(true)
            .open(&self.path)
            .map_err(|e| PwmEnableError::IOError { error: e })?;

        Ok(PwmEnable {
            file,
            path: self.path,
            pwmid: self.pwmid,
            _state: PhantomData,
        })
    }
}

impl WriteConfig for PwmEnable<PwmEnableReadWrite> {
    fn set_fan_mode(&mut self, mode: AsusNbWmiFanMode) -> Result<(), FanModeSetError> {
        let Err(e) = self.file.write_all_at(&[mode as u8], 0) else {
            return Ok(());
        };

        let raw_err = e.raw_os_error();

        let Some(err) = raw_err else {
            return Err(FanModeSetError::UnknownError { error: e.into() });
        };

        Err(match err {
            22 => FanModeSetError::IllegalFanModeValue { value: mode },
            5 => FanModeSetError::AcPowerRequired { error: e.into() },
            _ => FanModeSetError::UnknownError { error: e.into() },
        })
    }
}
