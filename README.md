# meh-asus

![Crates.io Version](https://img.shields.io/crates/v/meh_asus)
![Crates.io Size](https://img.shields.io/crates/size/meh_asus)

Abstraction over ASUS hardware configurations to control it programatically on Linux.

This can utilize any hardware's DEV_ID, to read and modify its configuration as defined by the user.

You can find the DEV_ID of the hardware you're interested in [ASUS WMI source code](https://github.com/torvalds/linux/blob/master/drivers/platform/x86/asus-wmi.c).


---

## Common Hardware

Some of the hardware dev id and their states are defined by default, serving as an example as well as direct abstraction over them.

> [!NOTE]
> Users can refer to them to use it themselves, or to create abstraction over some other hardware component of their choice. Also look at [examples](#usage-of-examples-given).

---

## Usage of examples given

- speed_change: Make you fan go from Auto to Fullspeed for fun. (no debugfs)
- toggle_fan_mode: Switches fan from Auto to FullSpeed and vice-versa.
- camera_modprobe: Enable/Disable the camera and utilize the camera led to indicate its status.
- kbd_brightness: Toggles brightness of keyboard blacklight.

### Run as superuser

With sudo: `sudo cargo r -r --example toggle_fan_mode`.

### Run like a shell script / keyboard shortcut

This is how `toggle_fan_mode` can be setup, `camera_modprobe` and other porgrams can be used in a similar fashion.

1. Compile using cargo `cargo b -r --example toggle_fan_mode`

2. Give root level execution permission

```bash
sudo chown root:root ./target/release/examples/toggle_fan_mode
sudo chmod 4005 ./target/release/examples/toggle_fan_mode
```

3. Run script from shell or set key binding in keyboard shortcut (say <kbd>Meta</kbd>+<kbd>;</kbd>)

> [!TIP]
> One would also like to enable notification for the changes. For notifications, make sure `libnotify` and `xargs` is installed on your system, and use the following command
> 
> ```bash
> /PATH/TO/BINARY/debugfs_fan_mode 2>&1 | xargs -I {} notify-send -a "meh-asus" "Fan Mode Switch" "{}"
> ```
>
> This is example command using fan_mode toggle (with debugfs).

---

## Running tests

> [!IMPORTANT]
> Use single thread, since all the configurations are essentially using file modification techniques, hence parallel execution might fail.

Your laptop may not support some of the tests execution, this way you get to know about what crate in-built hardware features you can use.

```bash
sudo cargo test --no-fail-fast -- --test-threads=1 --nocapture
```

---

## Note on Fan control

### debugfs (default)

Windows doesn't allow you to switch to fan modes such as `FullSpeed` when you are not connected to AC power. This module doesn't impose any such restriction, although the configuration may be rejected by the hardware itself (passing an error message), but you should still be careful yourself to handle such scenerio, else you may end up harming you laptop's battery life.

### pwm mode

There are more fan configuration and files which can be used to handle the fan, but I didn't need them.

Not all asus laptops are supported. I mean they are, if you create mapping yourself, the majority of code remains same, only file names and the byte mapping to fan mode changes. My laptop only had the `pwm1_enable` and `pwm2_enable` (cpu and gpu fans), so I didn't program for other possible fans (only `pwm{id}_enable` in general).

Refer https://wiki.archlinux.org/title/Fan_speed_control#ASUS_laptops

I would advise to use the debugfs abstraction over this, this is more of "manual" configuration.

---
