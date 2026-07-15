mod app_state;
mod battery;
mod colors;
mod commands;
mod display;
mod monitor;
mod sysinfo;

use app_state::AppState;
use std::sync::Arc;
use std::thread;

fn main() {
    let initial_battery_path = battery::detect_initial_path();
    let state = Arc::new(AppState::new(initial_battery_path));

    let monitor_state = Arc::clone(&state);
    thread::spawn(move || monitor::run(monitor_state));

    commands::run(state);
}