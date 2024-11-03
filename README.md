# meh-asus fan control

Abstraction over fan mode to control it programatically. Basically to switch fan modes on my asus laptop (linux) just like I can do natively on Windows.

---

This is meant only for Asus laptops, there are more fan configuration and files which can be used to handle the fan, but I didn't need them.

Not all asus laptops are supported. I mean they are, if you create mapping yourself, the majority of code remains same, only file names and the byte mapping to fan mode changes. My laptop only had the `pwm1_enable` and `pwm2_enable` (cpu and gpu fans), so I didn't program for other possible fans (only `pwm{id}_enable` in general).

Refer https://wiki.archlinux.org/title/Fan_speed_control#ASUS_laptops

---

## Usage of toggle_fan_mode / camera_modprobe

- toggle_fan_mode: Switches fan from Auto to FullSpeed and vice-versa.
- camera_modprobe: Enable/Disable the camera and utilize the camera led to indicate its status.

This is how `toggle_fan_mode` can be setup, `camera_modprobe` and other porgrams can be used in a similar fashion.

1. Compile using cargo `cargo b -r --example toggle_fan_mode`

2. Give root level execution permission

```bash
sudo chown root:root ./target/release/examples/toggle_fan_mode
sudo chmod 4005 ./target/release/examples/toggle_fan_mode
```

3. Run script from shell or set key binding in keyboard shortcut (say <kbd>Meta</kbd>+<kbd>;</kbd>)

---
