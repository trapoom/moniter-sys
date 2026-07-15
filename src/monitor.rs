use crate::app_state::AppState;
use crate::battery::get_battery;
use crate::colors::paint;
use crate::sysinfo::{get_cpu_temp, get_ram};
use std::io::{self, Write};
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub fn run(state: Arc<AppState>) {
    while state.running.load(Ordering::Relaxed) {
        let cpu = get_cpu_temp();
        let ram = get_ram();
        let bat = get_battery(&state.battery_path.lock().unwrap());

        let line = format!("[ CPU: {} | RAM: {} | BAT: {} ]", cpu, ram, bat);

        let mut out = io::stdout();
        write!(out, "\x1b[s").ok();
        write!(out, "\x1b[A\r\x1b[K").ok();
        write!(out, "{}", paint(&state, &line)).ok();
        write!(out, "\x1b[u").ok();
        out.flush().ok();

        thread::sleep(Duration::from_secs(1));
    }
}