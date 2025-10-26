# basic\_status\_message

A simple status bar message generator for [Sway](https://swaywm.org/). Example:
```
2025-10-26 13:53 🔆35% 🔋94%
```

Written by Madeleine Thompson, 2024-2025. This is in the public domain.

# Dependencies

Install Rust, `brightnessctl`, and an emoji font. On Debian-derived systems,
follow the instructions at https://rust-lang.org/learn/get-started/, then:
```
sudo apt update && sudo apt install -y brightnessctl fonts-noto-color-emoji
```

# Installation

```
git clone git@github.com:madeleineth/basic_status_message.git
cd basic_status_message
cargo install --path .
```
Then, put a block like the following in `.config/sway/config`, e.g.:
```
bar {
    position top
    status_command ~/.cargo/bin/basic_status_message
}
```
You will need to specify `--battery` or `--backlight` if your laptop uses
different paths than mine. The defaults work on a Framework 13 (AMD). On a
ThinkPad X1:
```
status_command ~/.cargo/bin/basic_status_message --battery /sys/class/power_supply/BAT0 --backlight /sys/class/backlight/intel_backlight
```
I also have these lines in `.config/sway/config` for adjusting brightness:
```
bindsym XF86MonBrightnessUp exec brightnessctl s +10%
bindsym XF86MonBrightnessDown exec brightnessctl s 10%-
```
