use anyhow::{Context, Error};
use meh_asus::pwm::fan::AsusNbWmiFanMode;
use meh_asus::pwm::pwm_enable::error::{FanModeReadError, InputReadError, LabelReadError};
use meh_asus::pwm::pwm_enable::traits::{PwmEnableState, PwmHardware, ReadConfig, WriteConfig};
use meh_asus::pwm::pwm_enable::{PwmEnable, PwmEnableReadOnly, PwmEnableReadWrite};
use thiserror::Error;

#[derive(Debug)]
struct FanConfiguration {
    pwm_id: u8,
    label: String,
    input: u16,
    mode: AsusNbWmiFanMode,
}

impl std::fmt::Display for FanConfiguration {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}: ({} Mode, {}rpm) [pwm{}]",
            self.label, self.mode, self.input, self.pwm_id
        )
    }
}

#[derive(Debug, Error)]
enum FanConfigurationError {
    #[error("{0}")]
    LabelReadError(LabelReadError),
    #[error("{0}")]
    InputReadError(InputReadError),
    #[error("{0}")]
    FanModeReadError(FanModeReadError),
}

fn get_config<X, T>(x: &T) -> Result<FanConfiguration, FanConfigurationError>
where
    T: PwmHardware<X> + ReadConfig,
    X: PwmEnableState,
{
    Ok(FanConfiguration {
        pwm_id: x.get_pwm_id(),
        label: x
            .get_label()
            .map_err(|e| FanConfigurationError::LabelReadError(e))?,
        input: x
            .get_input()
            .map_err(|e| FanConfigurationError::InputReadError(e))?,
        mode: x
            .get_fan_mode()
            .map_err(|e| FanConfigurationError::FanModeReadError(e))?,
    })
}

fn main() -> Result<(), Error> {
    let fans: Result<(PwmEnable<PwmEnableReadWrite>, PwmEnable<PwmEnableReadWrite>), ()> = {
        let fan1: PwmEnable<PwmEnableReadWrite> = PwmEnable::find_and_get(1)
            .context("fan1 hardware not found or failed to open config")?;
        let fan2: PwmEnable<PwmEnableReadWrite> = PwmEnable::find_and_get(2)
            .context("fan2 hardware not found or failed to open config")?;

        Ok((fan1, fan2))
    };

    if let Ok((mut fan1, mut fan2)) = fans {
        let fan1_config = get_config(&fan1).expect("Error reading fan1 config");

        // lets sync both fans to same mode
        let next_mode = get_next_mode(&fan1_config);

        let fan1_config = get_config(&fan1).expect("Error reading fan1 config");
        let fan2_config = get_config(&fan2).expect("Error reading fan1 config");

        fan1.set_fan_mode(next_mode)
            .context("Failed to switch to next fan mode")?;
        fan2.set_fan_mode(next_mode)
            .context("Failed to switch to next fan mode")?;

        println!(
            "Switched fan mode to next settings\n{}\n{}",
            fan1_config, fan2_config
        );
    } else {
        let fan1: PwmEnable<PwmEnableReadOnly> = PwmEnable::find_and_get(1)
            .context("fan1 hardware not found or failed to read config")?;
        let fan2: PwmEnable<PwmEnableReadOnly> = PwmEnable::find_and_get(2)
            .context("fan2 hardware not found or failed to read config")?;

        let fan1_config = get_config(&fan1).context("Error reading fan1 config")?;
        let fan2_config = get_config(&fan2).context("Error reading fan2 config")?;

        eprintln!(
            "{}\n{}\nFailed to open fan hardware as read-write.\nPlease run this program as root to control the fans.",
            fan1_config, fan2_config
        );
    }

    Ok(())
}

fn get_next_mode(curr_config: &FanConfiguration) -> AsusNbWmiFanMode {
    match curr_config.mode {
        AsusNbWmiFanMode::FullSpeed => AsusNbWmiFanMode::Auto,
        AsusNbWmiFanMode::Manual => AsusNbWmiFanMode::Auto,
        AsusNbWmiFanMode::Auto => AsusNbWmiFanMode::FullSpeed,
    }
}
