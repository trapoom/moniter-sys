use crate::app_state::AppState;

pub const RESET: &str = "\x1b[0m";

pub const NAMES: [&str; 8] = [
    "default", "red", "green", "yellow", "blue", "magenta", "cyan", "white",
];

fn code_for(name: &str) -> &'static str {
    match name {
        "default" => "\x1b[0m",
        "red" => "\x1b[91m",
        "green" => "\x1b[92m",
        "yellow" => "\x1b[93m",
        "blue" => "\x1b[94m",
        "magenta" => "\x1b[95m",
        "cyan" => "\x1b[96m",
        "white" => "\x1b[97m",
        _ => "\x1b[0m",
    }
}

pub fn is_valid(name: &str) -> bool {
    NAMES.contains(&name)
}

pub fn paint(state: &AppState, text: &str) -> String {
    let color = state.current_color.lock().unwrap();
    format!("{}{}{}", code_for(&color), text, RESET)
}