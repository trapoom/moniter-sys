use crate::app_state::AppState;
use crate::display::box_print;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

const POWER_SUPPLY_DIR: &str = "/sys/class/power_supply";

pub fn find_battery_paths() -> Vec<String> {
    let mut found = Vec::new();
    let entries = match fs::read_dir(POWER_SUPPLY_DIR) {
        Ok(e) => e,
        Err(_) => return found,
    };

    for entry in entries.flatten() {
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if name.starts_with("BAT") {
            let capacity = entry.path().join("capacity");
            if capacity.exists() {
                found.push(capacity.to_string_lossy().to_string());
            }
        }
    }
    found.sort();
    found
}

pub fn detect_initial_path() -> Option<String> {
    find_battery_paths().into_iter().next()
}

pub fn get_battery(path: &Option<String>) -> String {
    let path = match path {
        Some(p) => p,
        None => return "not set".to_string(),
    };
    match fs::read_to_string(path) {
        Ok(content) => format!("{}%", content.trim()),
        Err(_) => "invalid path".to_string(),
    }
}

pub fn show_bat_details(state: &AppState) {
    let mut lines = Vec::new();
    let path_guard = state.battery_path.lock().unwrap();

    match path_guard.as_ref() {
        None => lines.push("battery path not set, use 'set'".to_string()),
        Some(path) => {
            let base_path = Path::new(path).parent().unwrap_or(Path::new("/"));
            for node in ["status", "health", "technology", "voltage_now"] {
                let node_path: PathBuf = base_path.join(node);
                if node_path.exists() {
                    if let Ok(content) = fs::read_to_string(&node_path) {
                        let label = to_title_case(node);
                        lines.push(format!("{}: {}", label, content.trim()));
                    }
                }
            }
        }
    }
    drop(path_guard);

    if lines.is_empty() {
        lines.push("Cannot read Battery details".to_string());
    }
    box_print(state, "Battery Details", &lines);
}

pub fn setup_battery_path(state: &AppState) {
    let mut lines = vec!["Example: /sys/class/power_supply/BAT0/capacity".to_string()];
    for (i, p) in find_battery_paths().iter().enumerate() {
        lines.push(format!("{}. {}", i + 1, p));
    }
    box_print(state, "Setup Battery Path", &lines);

    print!("path (enter to skip): ");
    io::stdout().flush().ok();
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        return;
    }
    let new_path = input.trim();

    if new_path.is_empty() {
        println!("{}", crate::colors::paint(state, "skipped"));
        return;
    }

    if Path::new(new_path).exists() {
        *state.battery_path.lock().unwrap() = Some(new_path.to_string());
        println!(
            "{}",
            crate::colors::paint(state, &format!("set: {}", new_path))
        );
    } else {
        println!("{}", crate::colors::paint(state, "path not found"));
    }
}

fn to_title_case(node: &str) -> String {
    node.split('_')
        .map(|part| {
            let mut c = part.chars();
            match c.next() {
                Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}