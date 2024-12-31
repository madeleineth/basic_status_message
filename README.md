# basic\_status\_message

Written by Madeleine Thompson, 2024. This is in the public domain.

To install:
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
different paths than mine.
