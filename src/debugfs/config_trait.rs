//! Config trait for enums to be used as configuration for hardware components
use std::fmt::Debug;

/// A trait convert the state enum to a valid configuration, to write to configuration file of the hardware.
///
/// Refer to "common_hardware" module and examples, and
/// [ASUS WMI source code](https://github.com/torvalds/linux/blob/master/drivers/platform/x86/asus-wmi.c)
/// to fully understand the possible configuration.
///
/// Usually ranges from 0 upto possible states of the hardware.
///
///
/// Example:
/// ```rust
/// use meh_asus::Config;
/// use meh_asus::error::StateError;
/// 
/// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// pub enum LedState {
///     Off,
///     On,
/// }
///
/// impl Config for LedState {
///     fn to_config(&self) -> String {
///         match self {
///             LedState::Off => String::from("0"),
///             LedState::On => String::from("1"),
///         }
///     }
/// }
/// 
/// impl TryFrom<u64> for LedState {
///     type Error = StateError;
///     fn try_from(value: u64) -> Result<Self, Self::Error> {
///         match value {
///             0 => Ok(LedState::Off),
///             1 => Ok(LedState::On),
///             _ => Err(StateError::NotPossibleState { value }),
///         }
///     }
///  }
/// ```
pub trait Config: TryFrom<u64> + Debug + Copy {
    /// Configuration string corresponding to the type.
    fn to_config(&self) -> String;
}

macro_rules! impl_config {
    ($($t:ty),*) => {
        $(
            impl Config for $t {
                fn to_config(&self) -> String {
                    self.to_string()
                }
            }
        )*
    };
}

impl_config!(u64, u8, u16, u32, i8, i16, i32, i64);

/// Automatically make a State enum and implement
/// [TryFrom\<u64\>](core::convert::TryFrom) and [Config](Config)
/// with its decleration.
/// 
/// Optionally pass a enum repr type (such as `u8`, `u32`) [defaults to `u64`]
///
/// Works in most cases, except few where different masks are involved
/// such as keyboard blacklight.
///
/// Use [create_kbd_brightness_enum](crate::create_kbd_brightness_enum)
/// for keyboard blacklight state.
///
/// Example:
/// ```rust
/// use meh_asus::auto_impl_config;
/// use meh_asus::Config;
/// use meh_asus::error::StateError;
///
/// auto_impl_config!(HardwareState, StateA = 0, StateB = 1, StateC = 3, StateD = 2);
/// auto_impl_config!(HardwareStateU8, u8, StateA = 0, StateB = 1, StateC = 3, StateD = 2);
/// ```
#[macro_export]
macro_rules! auto_impl_config {
    ($enum_name:ident, $type: ty, $($name:ident = $value:expr),*) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        #[repr($type)]
        pub enum $enum_name {
            $($name = $value),*
        }

        impl TryFrom<u64> for $enum_name {
            type Error = StateError;

            fn try_from(value: u64) -> Result<Self, Self::Error> {
                match value {
                    $( $value => Ok($enum_name::$name), )*
                    _ => Err(StateError::NotPossibleState { value }),
                }
            }
        }

        impl Config for $enum_name {
            fn to_config(&self) -> String {
                (*self as $type).to_string()
            }
        }
    };
    ($enum_name:ident, $($name:ident = $value:expr),*) => {
        auto_impl_config!($enum_name, u64, $($name = $value),*);
    };
}
