use anyhow::{Context, Error};
use meh_asus::debugfs::common_hardware::{camera_led::CAMERA_LED, led_state::LedState};
use notify_rust::Notification;

fn is_superuser() -> bool {
    unsafe { libc::geteuid() == 0 }
}

const CAMERA_MODULE: &str = "uvcvideo";

fn main() -> Result<(), Error> {
    if !is_superuser() {
        eprintln!("This program must be run as root");

        Notification::new()
            .appname("Asus Camera Module")
            .summary("Camera module switch failed")
            .body("This program must be run as root for the camera module to be enabled/disabled")
            .show()
            .context("Failed to show notification")?;

        return Ok(());
    }

    let camera_led = CAMERA_LED;
    let curr_state = camera_led.read()?;

    match curr_state {
        LedState::On => {
            let output = std::process::Command::new("modprobe")
                .arg(CAMERA_MODULE)
                .output()
                .context("Failed to run modprobe")?;

            if output.status.success() {
                println!("Camera module {} has been enabled", CAMERA_MODULE,);
                camera_led.apply(LedState::Off)?;
            } else {
                eprintln!(
                    "Failed to enable camera module {}:\n{}",
                    CAMERA_MODULE,
                    String::from_utf8_lossy(&output.stderr)
                );
                camera_led.apply(LedState::Off)?;
            }
        }
        LedState::Off => {
            let output = std::process::Command::new("rmmod")
                .arg("-f")
                .arg(CAMERA_MODULE)
                .output()
                .context("Failed to run rmmod")?;

            if output.status.success() {
                println!("Camera module {} has been disabled", CAMERA_MODULE);
                camera_led.apply(LedState::On)?;
            } else {
                eprintln!(
                    "Failed to disable camera module {}:\n{}",
                    CAMERA_MODULE,
                    String::from_utf8_lossy(&output.stderr),
                );
                camera_led.apply(LedState::Off)?;
            }
        }
    }

    println!("Camera LED status: {:?}", camera_led.read());
    Ok(())
}
