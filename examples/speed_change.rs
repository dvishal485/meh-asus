use std::{thread::sleep, time::Duration};

use anyhow::Result;
use meh_asus::pwm_enable::{PwmEnableAbstraction, AsusNbWmiFanMode};

macro_rules! print_config {
    ($($x:expr),+ $(,)?) => {
        $( println!("{} => {} ({} rpm)", $x.get_label().unwrap(), $x.get_fan_mode().unwrap(), $x.get_input().unwrap()); )+
    };
}

fn main() -> Result<()> {
    let fan1 = PwmEnableAbstraction::find_and_get_read_only(1)?;
    let fan2 = PwmEnableAbstraction::find_and_get_read_only(2)?;

    // (_, _)
    print_config!(fan1, fan2);


    let mut fan1 = PwmEnableAbstraction::find_and_get_read_write(1)?;
    let mut fan2 = PwmEnableAbstraction::find_and_get_read_write(2)?;

    // (_, _)
    print_config!(fan1, fan2);

    fan1.set_fan_mode(AsusNbWmiFanMode::FullSpeed)?;
    
    // (FullSpeed, _)
    print_config!(fan1, fan2);
    sleep(Duration::from_secs(5));

    fan2.set_fan_mode(AsusNbWmiFanMode::FullSpeed)?;

    // (FullSpeed, FullSpeed)
    print_config!(fan1, fan2);
    sleep(Duration::from_secs(5));

    fan1.set_fan_mode(AsusNbWmiFanMode::Auto)?;
    
    // (Auto, FullSpeed)
    print_config!(fan1, fan2);
    sleep(Duration::from_secs(5));

    fan2.set_fan_mode(AsusNbWmiFanMode::Auto)?;

    // (Auto, Auto)
    print_config!(fan1, fan2);

    Ok(())
}