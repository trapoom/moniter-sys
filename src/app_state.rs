use std::sync::atomic::AtomicBool;
use std::sync::Mutex;

pub struct AppState {
    pub running: AtomicBool,
    pub current_color: Mutex<String>,
    pub battery_path: Mutex<Option<String>>,
}

impl AppState {
    pub fn new(initial_battery_path: Option<String>) -> Self {
        AppState {
            running: AtomicBool::new(true),
            current_color: Mutex::new("red".to_string()),
            battery_path: Mutex::new(initial_battery_path),
        }
    }
}