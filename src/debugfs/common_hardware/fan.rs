use crate::debugfs::{config::Hardware, Config, error::HardwareError};

pub const DEV_ID: u64 = 0x110019;

pub const FAN: Hardware<FanMode> = Hardware::new(DEV_ID);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum FanMode {
    Standard = 0,
    Whispher = 1,
    Performace = 2,
    FullSpeed = 3,
}

impl TryFrom<u64> for FanMode {
    type Error = HardwareError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value as u8 {
            0 => Ok(FanMode::Standard),
            1 => Ok(FanMode::Whispher),
            2 => Ok(FanMode::Performace),
            3 => Ok(FanMode::FullSpeed),
            _ => Err(HardwareError::NotPossibleState { value }),
        }
    }
}

impl Config for FanMode {
    fn to_config(&self) -> String {
        (*self as u8).to_string()
    }
}

#[test]
fn fan_modes() {
    use std::thread::sleep;
    use std::time::Duration;

    let fan = FAN;

    let initial_state = fan.read().expect("there should be a current state of fan");

    // set fan to standard mode
    fan.apply(FanMode::Standard)
        .expect("fan should be set to standard mode");

    assert_eq!(fan.read().unwrap(), FanMode::Standard);

    // sleep for 2 seconds
    sleep(Duration::from_secs(2));

    // restore initial fan mode
    fan.apply(initial_state)
        .expect("fan should be switched to initial state");

    assert_eq!(
        fan.read().unwrap(),
        initial_state,
        "Failed to revert to initial state"
    );
}
