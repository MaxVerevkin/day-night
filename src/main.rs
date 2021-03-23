mod day_night;

use clap::*;
use std::process::Command;
use std::process::Stdio;
use std::thread;
use std::time::Duration;

use day_night::DayNight;

fn main() {
    // Init clap
    let matches = app_from_crate!()
        .arg(
            Arg::with_name("interval")
                .help("The interval between updates in seconds")
                .default_value("600")
                .takes_value(true)
                .long("interval"),
        )
        .arg(
            Arg::with_name("latitude")
                .help("Your geographical latitude")
                .takes_value(true)
                .required(true)
                .long("lat"),
        )
        .arg(
            Arg::with_name("longitude")
                .help("Your geographical longitude")
                .takes_value(true)
                .required(true)
                .long("lon"),
        )
        .arg(
            Arg::with_name("day")
                .help("A shell command to run in day time")
                .takes_value(true)
                .long("day"),
        )
        .arg(
            Arg::with_name("night")
                .help("A shell command to run in night time")
                .takes_value(true)
                .long("night"),
        )
        .get_matches();

    // Get arguments
    let interval: u64 = matches
        .value_of("interval")
        .unwrap()
        .parse()
        .expect("interval shuld be a positive integer");
    let interval = Duration::from_secs(interval);
    let latitude: f64 = matches
        .value_of("latitude")
        .unwrap()
        .parse()
        .expect("latitude shuld be a number");
    let longitude: f64 = matches
        .value_of("longitude")
        .unwrap()
        .parse()
        .expect("longitude shuld be a number");
    let day = matches.value_of("day");
    let night = matches.value_of("night");

    loop {
        let state = DayNight::current(latitude, longitude);
        let cmd = match state {
            DayNight::Day => day,
            DayNight::Night => night,
        };
        if let Some(cmd) = cmd {
            dbg!(cmd);
            spawn_shell_async(cmd);
        }
        thread::sleep(interval);
    }
}

fn spawn_shell_async(cmd: &str) {
    let mut child = Command::new("sh")
        .args(&["-c", cmd])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .spawn()
        .unwrap();
    thread::Builder::new()
        .name("subprocess".into())
        .spawn(move || child.wait())
        .unwrap();
}
