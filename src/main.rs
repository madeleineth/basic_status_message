use chrono::Local;
use clap::Parser;
use inotify::{Inotify, WatchMask};
use nix::poll::{poll, PollFd, PollFlags};
use std::cmp::min;
use std::fs;
use std::os::fd::AsFd;
use std::path::{Path, PathBuf};
use std::process;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

const BATTERY_EMOJI: &str = "\u{1F50B}";
const BRIGHTNESS_EMOJI: &str = "\u{1F506}";

fn read_int_from_file(path: &Path) -> Option<i32> {
    match fs::read_to_string(path) {
        Ok(contents) => match contents.trim().parse::<i32>() {
            Ok(i) => Some(i),
            Err(_) => None,
        },
        Err(_) => None,
    }
}

fn battery_pct(path: &Path) -> String {
    match read_int_from_file(path) {
        Some(i) => format!("{}%", i),
        None => "???%".to_string(),
    }
}

fn brightness_pct(brightness_path: &Path, max_brightness_path: &Path) -> String {
    let actual = read_int_from_file(brightness_path);
    let max = read_int_from_file(max_brightness_path);
    match (actual, max) {
        (Some(a), Some(m)) => format!("{}%", a * 100 / m),
        _ => "???%".to_string(),
    }
}

fn print_status_message(battery_path: &Path, brightness_path: &Path, max_brightness_path: &Path) {
    let now = Local::now();
    let tm = now.format("%Y-%m-%d %H:%M");
    let bright = brightness_pct(brightness_path, max_brightness_path);
    let bat = battery_pct(battery_path);
    println!(
        "{} {}{} {}{}",
        tm, BRIGHTNESS_EMOJI, bright, BATTERY_EMOJI, bat
    );
}

#[derive(Parser, Debug)]
#[command(
    name = "basic_status_message",
    about = "to be used as a status_command in .config/sway/config"
)]
struct Args {
    #[arg(
        long,
        value_name = "BATTERY_PATH",
        default_value = "/sys/class/power_supply/BAT1",
        required = false
    )]
    battery: PathBuf,

    #[arg(
        long,
        value_name = "BACKLIGHT_PATH",
        default_value = "/sys/class/backlight/amdgpu_bl0",
        required = false
    )]
    backlight: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let battery_path = args.battery.join("capacity");
    let brightness_path = args.backlight.join("actual_brightness");
    let max_brightness_path = args.backlight.join("max_brightness");
    for p in [&battery_path, &brightness_path, &max_brightness_path].iter() {
        if !p.exists() {
            eprintln!("{:?} does not exist.", p);
            process::exit(1);
        }
    }

    let mut inotify = Inotify::init()?;
    inotify.watches().add(&battery_path, WatchMask::MODIFY)?;
    inotify.watches().add(&brightness_path, WatchMask::MODIFY)?;
    loop {
        print_status_message(
            battery_path.as_path(),
            brightness_path.as_path(),
            max_brightness_path.as_path(),
        );
        // Wait until either the battery or brightness changes or 1ms after the
        // next even minute, whichever happens first.
        let ms = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();
        let sleep_ms = min(60_001 - (ms % 60_000), u16::MAX as u128) as u16;
        match {
            let mut poll_fds = [PollFd::new(inotify.as_fd(), PollFlags::POLLIN)];
            poll(&mut poll_fds, sleep_ms)
        } {
            Ok(0) => {}
            Ok(_) => {
                let mut buffer = [0; 1024];
                inotify.read_events_blocking(&mut buffer)?;
                ()
            }
            Err(e) => {
                eprintln!("Error polling: {:?}", e);
                process::exit(1);
            }
        };
    }
}
