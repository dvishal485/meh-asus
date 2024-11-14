//! ASUS Laptop Fan modes similar to windows configuration of fan.
//!
//! On windows it if updated using <kbd>Fn</kbd> + <kbd>F</kbd> key.
//!
//! You can use this module to obtain similar functionality.
//! Refer to README for usage.
use crate::{
    auto_impl_config,
    debugfs::{Config, Hardware},
    error::StateError,
};

pub const DEV_ID: u64 = 0x110019;

pub const fn get() -> Hardware<FanMode> {
    Hardware::new(DEV_ID)
}

auto_impl_config!(
    FanMode,
    u8,
    Standard = 0,
    Whispher = 1,
    Performace = 2,
    FullSpeed = 3
);

#[test]
fn fan_modes() {
    use std::thread::sleep;
    use std::time::Duration;

    let fan = get();

    let initial_state = fan.read().expect("there should be a current state of fan");

    // set fan to standard mode
    fan.apply(FanMode::Standard)
        .expect("fan should be set to standard mode");

    assert_eq!(fan.read().unwrap(), FanMode::Standard);

    // sleep for 2 seconds
    sleep(Duration::from_secs(2));

    // restore initial fan mode
    fan.apply(initial_state)
        .expect("fan should be switched to initial state");

    assert_eq!(
        fan.read().unwrap(),
        initial_state,
        "Failed to revert to initial state"
    );
}
