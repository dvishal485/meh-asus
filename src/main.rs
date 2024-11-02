mod debugfs;
use std::{thread::sleep, time::Duration};

use debugfs::common_hardware::camera_led::CameraLedState;
use debugfs::common_hardware::camera_led::CAMERA_LED;

fn main() {
    let camera_led = CAMERA_LED;

    let value = camera_led.read().unwrap();
    println!("{:?}", value);

    sleep(Duration::from_secs(5));

    camera_led.apply(CameraLedState::On).unwrap();

    let value = camera_led.read().unwrap();
    println!("{:?}", value);

    sleep(Duration::from_secs(5));

    camera_led.apply(CameraLedState::Off).unwrap();

    let value = camera_led.read().unwrap();
    println!("{:?}", value);
}
