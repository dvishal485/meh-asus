//! ASUS_WMI_DEVID_MICMUTE_LED
//! 
//! Audio Button LED Key (F9 on most Asus Laptop)

pub use super::led_state::LedState;
use crate::debugfs::Hardware;

pub const DEV_ID: u64 = 0x00040017;
pub const fn get() -> Hardware<LedState> {
    Hardware::new(DEV_ID)
}

#[test]
fn mic_led() {
    let mic_led = get();

    let initial_state = mic_led
        .read()
        .expect("there should be a current state of mic led");

    // turn on led
    mic_led
        .apply(LedState::On)
        .expect("mic led should be turned on");
    assert_eq!(mic_led.read().unwrap(), LedState::On);

    // turn off led
    mic_led
        .apply(LedState::Off)
        .expect("mic led should be turned off");
    assert_eq!(mic_led.read().unwrap(), LedState::Off);

    // return to initial state
    mic_led
        .apply(initial_state)
        .expect("mic led should be switched to initial state");

    assert_eq!(
        mic_led.read().unwrap(),
        initial_state,
        "Failed to revert to initial state"
    );
}
