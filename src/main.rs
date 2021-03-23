mod day_night;

use std::process::Command;
use std::process::Stdio;
use std::thread;
use std::time::Duration;

use day_night::DayNight;

pub const LAT: f64 = 30.;
pub const LON: f64 = 30.;
const DAY_CMD: &str = "ln -fs ~/.config/nvim/theme-light.vim ~/.config/nvim/theme.vim; ln -fs ~/.config/kitty/kitty-themes/themes/Material.conf ~/.config/kitty/theme.conf";
const NIGHT_CMD: &str = "ln -fs ~/.config/nvim/theme-dark.vim ~/.config/nvim/theme.vim; ln -fs ~/.config/kitty/kitty-themes/themes/MaterialDark.conf ~/.config/kitty/theme.conf";
const INTERVAL_SEC: u64 = 10;

fn main() {
    loop {
        let state = DayNight::current();
        match state {
            DayNight::Day => spawn_shell_async(DAY_CMD),
            DayNight::Night => spawn_shell_async(NIGHT_CMD),
        }
        thread::sleep(Duration::from_secs(INTERVAL_SEC));
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
