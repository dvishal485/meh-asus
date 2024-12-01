use std::process::ExitCode;

use fan::FanMode;
use meh_asus::common_hardware::fan;

fn main() -> ExitCode {
    let fan = fan::get();
    let next_fan_mode = match fan.read() {
        Ok(curr_fan_mode) => match curr_fan_mode {
            FanMode::Whispher => FanMode::Standard,
            FanMode::Standard => FanMode::Performace,
            FanMode::Performace => FanMode::FullSpeed,
            FanMode::FullSpeed => FanMode::Whispher,
        },
        Err(_) => FanMode::Standard,
    };

    if let Err(e) = fan.apply(next_fan_mode) {
        eprintln!("Failed to apply fan mode!\n{:?}", e);
        ExitCode::FAILURE
    } else {
        if let Ok(true) = fan.read().map(|curr_mode| curr_mode == next_fan_mode) {
            println!("Switched fan mode to {:?}", next_fan_mode);
            ExitCode::SUCCESS
        } else {
            eprintln!("Failed to switch fan mode!");
            ExitCode::FAILURE
        }
    }
}
