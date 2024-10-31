use std::{thread::sleep, time::Duration};

use meh_asus::pwm_enable::{error::PwmEnableError, AsusNbWmiFanMode, PwmEnableAbstraction};

macro_rules! print_config {
    ($($x:expr),+ $(,)?) => {
        $( println!("{} => {} ({} rpm)", $x.get_label().unwrap(), $x.get_fan_mode().unwrap(), $x.get_input().unwrap()); )+
        println!();
    };
}

fn main() -> Result<(), PwmEnableError> {
    let fan1 = PwmEnableAbstraction::find_and_get_read_only(1)?;
    let fan2 = PwmEnableAbstraction::find_and_get_read_only(2)?;

    // (_, _)
    print_config!(fan1, fan2);

    let mut fan1 = PwmEnableAbstraction::find_and_get_read_write(1)?;
    let mut fan2 = PwmEnableAbstraction::find_and_get_read_write(2)?;

    // (_, _)
    print_config!(fan1, fan2);

    if let Err(e) = fan1.set_fan_mode(AsusNbWmiFanMode::FullSpeed) {
        eprintln!("Failed to set fan1 to FullSpeed: {}", e);
    }

    // (FullSpeed, _)
    print_config!(fan1, fan2);
    sleep(Duration::from_secs(5));

    if let Err(e) = fan2.set_fan_mode(AsusNbWmiFanMode::FullSpeed) {
        eprintln!("Failed to set fan2 to FullSpeed: {}", e);
    }

    // (FullSpeed, FullSpeed)
    print_config!(fan1, fan2);
    sleep(Duration::from_secs(5));

    if let Err(e) = fan1.set_fan_mode(AsusNbWmiFanMode::Auto) {
        eprintln!("Failed to set fan1 to Auto: {}", e);
    }

    // (Auto, FullSpeed)
    print_config!(fan1, fan2);
    sleep(Duration::from_secs(5));

    if let Err(e) = fan2.set_fan_mode(AsusNbWmiFanMode::Auto) {
        eprintln!("Failed to set fan2 to Auto: {}", e);
    }

    // (Auto, Auto)
    print_config!(fan1, fan2);

    Ok(())
}
