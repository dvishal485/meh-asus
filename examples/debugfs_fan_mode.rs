use anyhow::Error;
use fan::FanMode;
use meh_asus::common_hardware::fan;

fn main() -> Result<(), Error> {
    let fan = fan::get();
    let next_fan_mode = match fan.read()? {
        FanMode::Whispher => FanMode::Standard,
        FanMode::Standard => FanMode::Performace,
        FanMode::Performace => FanMode::FullSpeed,
        FanMode::FullSpeed => FanMode::Whispher,
    };

    fan.apply(next_fan_mode)?;

    assert_eq!(fan.read()?, next_fan_mode, "Failed to switch fan mode");
    println!("Switched fan mode to {:?}", next_fan_mode);

    Ok(())
}
