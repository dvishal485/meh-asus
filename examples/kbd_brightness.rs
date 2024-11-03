use meh_asus::create_kbd_brightness_enum;
use meh_asus::debugfs::common_hardware::kbd_blacklight::DEV_ID as KBD_DEV_ID;
use meh_asus::debugfs::Hardware;
use meh_asus::debugfs::{error::HardwareError, Config};

fn main() -> Result<(), HardwareError> {
    create_kbd_brightness_enum!(State, Off = 0, Low = 1, Medium = 2, High = 3);

    let kbd_blight: Hardware<State> = Hardware::new(KBD_DEV_ID);

    let next_state = match kbd_blight.read()? {
        State::Off => State::Low,
        State::Low => State::Medium,
        State::Medium => State::High,
        State::High => State::Off,
    };

    kbd_blight.apply(next_state)
}
