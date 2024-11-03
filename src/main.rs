mod debugfs;
use std::{thread::sleep, time::Duration};

use debugfs::common_hardware::camera_led::{CAMERA_LED, CameraLedState};
use debugfs::common_hardware::fan::{FAN,FanMode};

fn main() {
    let camera_led = CAMERA_LED;

    let value = camera_led.read_dsts();
    println!("DSTS {:?}\n", value);

    camera_led.apply(CameraLedState::On).unwrap();

    let value = camera_led.read_stale();
    println!("{:?}\n", value);

    let value = camera_led.read_dsts();
    println!("DSTS {:?}\n", value);

    sleep(Duration::from_secs(5));

    camera_led.apply(CameraLedState::Off).unwrap();

    let value = camera_led.read_stale();
    println!("{:?}\n", value);

    let value = camera_led.read_dsts();
    println!("DSTS {:?}\n", value);

    let fan = FAN ;

    let value = fan.read_dsts();
    println!("DSTS {:?}\n", value);

    fan.apply(FanMode::Performace).unwrap();

    let value = fan.read_dsts();
    println!("DSTS {:?}\n", value);

    let value = fan.read_stale();
    println!("{:?}\n", value);
}
