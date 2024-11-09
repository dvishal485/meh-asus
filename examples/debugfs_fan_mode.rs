use meh_asus::common_hardware::fan::{FanMode, FAN};
use meh_asus::error::HardwareError;

fn main() -> Result<(), HardwareError> {
    let next_fan_mode = match FAN.read()? {
        FanMode::Whispher => FanMode::Standard,
        FanMode::Standard => FanMode::Performace,
        FanMode::Performace => FanMode::FullSpeed,
        FanMode::FullSpeed => FanMode::Whispher,
    };

    FAN.apply(next_fan_mode)?;

    assert_eq!(FAN.read()?, next_fan_mode, "Failed to switch fan mode");
    println!("Switched fan mode to {:?}", next_fan_mode);

    Ok(())
}
