//! ASUS_WMI_DEVID_KBD_BACKLIGHT
//!
//! Asus Keyboard blacklight control with a given number of blacklight mode settings
//! using [create_kbd_brightness_enum](crate::create_kbd_brightness_enum) utility macro.

pub const DEV_ID: u64 = 0x00050021;

/// Use this macro to create an enum for keyboard backlight
///
/// Macro should always start with an off state `Off = 0` and the rest of the states can be defined as needed.
///
/// Example usage:
///
/// ```rust
/// // these imports are required for the macro to work
/// use meh_asus::create_kbd_brightness_enum;
/// use meh_asus::common_hardware::kbd_blacklight::DEV_ID as KBD_DEV_ID;
/// use meh_asus::Hardware;
/// use meh_asus::{error::StateError, Config};
///
/// create_kbd_brightness_enum!(KbdState, Off = 0, Low = 1, Medium = 2, High = 3);
///
/// let kbd_blight: Hardware<KbdState> = Hardware::new(KBD_DEV_ID);
/// kbd_blight.apply(KbdState::Medium).unwrap();
/// ```
#[macro_export]
macro_rules! create_kbd_brightness_enum {
    ($enum_name:ident, $off_state: ident = 0, $($name:ident = $value:expr),*) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        #[repr(u64)]
        pub enum $enum_name {
            $off_state = 0,
            // https://github.com/torvalds/linux/blob/3e5e6c9900c3d71895e8bdeacfb579462e98eba1/drivers/platform/x86/asus-wmi.c#L1544-L1549
            $($name = 0x80 | (0x7F & $value)),*
        }

        impl TryFrom<u64> for $enum_name {
            type Error = StateError;

            fn try_from(value: u64) -> Result<Self, Self::Error> {
                match value {
                    0 => Ok($enum_name::$off_state),
                    $( $value => Ok($enum_name::$name), )*
                    _ => Err(StateError::NotPossibleState { value }),
                }
            }
        }

        impl Config for $enum_name {
            fn to_config(&self) -> String {
                (*self as u64).to_string()
            }
        }
    };
}

#[cfg(test)]
mod test {
    #[test]
    fn kbd_backlight() {
        use crate::create_kbd_brightness_enum;
        use crate::debugfs::common_hardware::kbd_blacklight::DEV_ID;
        use crate::debugfs::Hardware;
        use crate::debugfs::{error::StateError, Config};
        use std::{thread::sleep, time::Duration};

        create_kbd_brightness_enum!(KbdBrightness, Off = 0, Low = 1, Medium = 2, High = 3);

        let kbd_backlight = Hardware::new(DEV_ID);

        macro_rules! kbd_set {
            ($kbd_backlight: ident, $val: expr) => {
                kbd_backlight
                    .apply($val)
                    .expect("keyboard backlight should be set");

                let curr_state = $kbd_backlight.read();
                println!("Set State: {curr_state:?}");
                assert_eq!(curr_state.unwrap(), $val, "Expected config to be set");
            };
            ($val: expr) => {
                kbd_set!(kbd_backlight, $val);
            };
        }

        let initial_state = kbd_backlight
            .read()
            .expect("there should be a current state of keyboard backlight");

        println!("Initial state: {initial_state:?}");

        kbd_set!(KbdBrightness::Low);
        sleep(Duration::from_secs(2));

        kbd_set!(KbdBrightness::Medium);
        sleep(Duration::from_secs(2));

        kbd_set!(KbdBrightness::High);
        sleep(Duration::from_secs(2));

        kbd_set!(KbdBrightness::Off);
        sleep(Duration::from_secs(2));

        kbd_set!(KbdBrightness::Low);
        sleep(Duration::from_secs(2));

        // return to initial state
        kbd_set!(initial_state);
        sleep(Duration::from_secs(2));

        println!("Final state: {:?}", kbd_backlight.read());
        sleep(Duration::from_secs(5));

        assert_eq!(kbd_backlight.read().unwrap(), initial_state, "Initial state was not set!");
        println!("Success");
    }
}
