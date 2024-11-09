// ASUS_WMI_DEVID_CAMERA_LED
pub use super::led_state::LedState;
use crate::debugfs::Hardware;

pub const DEV_ID: u64 = 0x00060079;
pub const fn get() -> Hardware<LedState> {
    Hardware::new(DEV_ID)
}

#[test]
fn camera_led() {
    use libc::geteuid;
    let camera_led = get();
    
    if unsafe { geteuid() } != 0 {
        panic!("This test must be run as root");
    }

    let initial_state = camera_led
        .read()
        .expect("there should be a current state of camera led");

    // turn on led
    camera_led
        .apply(LedState::On)
        .expect("camera led should be turned on");
    assert_eq!(camera_led.read().unwrap(), LedState::On);

    // turn off led
    camera_led
        .apply(LedState::Off)
        .expect("camera led should be turned off");
    assert_eq!(camera_led.read().unwrap(), LedState::Off);

    // return to initial state
    camera_led
        .apply(initial_state)
        .expect("camera led should be switched to initial state");

    assert_eq!(
        camera_led.read().unwrap(),
        initial_state,
        "Failed to revert to initial state"
    );
}
