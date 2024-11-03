pub use super::led_state::LedState;
use crate::debugfs::config::Hardware;

pub const DEV_ID: u64 = 0x00040017;
pub const MIC_LED: Hardware<LedState> = Hardware::new(DEV_ID);

#[test]
fn test_mic_led() {
    use libc::geteuid;
    let mic_led = MIC_LED;

    if unsafe { geteuid() } != 0 {
        panic!("This test must be run as root");
    }

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
