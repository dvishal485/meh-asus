mod debugfs;
use std::{thread::sleep, time::Duration};

use debugfs::common_hardware::camera_led::{CAMERA_LED, CameraLedState};
use debugfs::common_hardware::fan::{FAN,FanMode};

fn main() {
    let camera_led = CAMERA_LED;

    let value = camera_led.read();
    println!("{:?}\n", value);

    camera_led.apply(CameraLedState::On).unwrap();

    let value = camera_led.read();
    println!("{:?}\n", value);

    sleep(Duration::from_secs(5));

    camera_led.apply(CameraLedState::Off).unwrap();

    let value = camera_led.read();
    println!("{:?}\n", value);

    let fan = FAN ;

    let value = fan.read();
    println!("{:?}\n", value);

    sleep(Duration::from_secs(5));

    fan.apply(FanMode::Performace).unwrap();

    let value = fan.read();
    println!("{:?}\n", value);
}
