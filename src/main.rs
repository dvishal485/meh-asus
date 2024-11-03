mod debugfs;
use std::{thread::sleep, time::Duration};

use debugfs::common_hardware::camera_led::{CameraLedState, CAMERA_LED};
use debugfs::common_hardware::fan::{FanMode, FAN};

fn main() {
    let mut camera_led = CAMERA_LED;

    println!("Current state of camera_led");
    let value = camera_led.read();
    println!("{:?}\n", value);

    sleep(Duration::from_secs(5));
    
    println!("Applying On state to camera_led");
    camera_led.apply(CameraLedState::On).unwrap();

    let value = camera_led.read();
    println!("{:?}\n", value);

    sleep(Duration::from_secs(5));

    println!("Applying Off state to camera_led");
    camera_led.apply(CameraLedState::Off).unwrap();

    let value = camera_led.read();
    println!("{:?}\n", value);

    let mut fan = FAN;

    println!("Current state of fan");
    let value = fan.read();
    println!("{:?}\n", value);

    sleep(Duration::from_secs(5));

    println!("Applying Performace state to fan");
    fan.apply(FanMode::Performace).unwrap();

    let value = fan.read();
    println!("{:?}\n", value);
}
