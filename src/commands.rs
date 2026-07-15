use crate::app_state::AppState;
use crate::battery::{setup_battery_path, show_bat_details};
use crate::colors::{self, paint};
use crate::display::box_print;
use std::fs;
use std::io::{self, Write};
use std::sync::atomic::Ordering;
use std::sync::Arc;

pub fn show_cpu_details(state: &AppState) {
    let mut lines = Vec::new();
    match fs::read_to_string("/proc/cpuinfo") {
        Ok(content) => {
            for line in content.lines().take(20) {
                if line.contains("model name") || line.contains("cpu MHz") || line.contains("processor") {
                    lines.push(line.trim().to_string());
                }
            }
        }
        Err(_) => lines.push("Cannot read CPU details".to_string()),
    }
    box_print(state, "CPU Details", &lines);
}

pub fn show_ram_details(state: &AppState) {
    let mut lines = Vec::new();
    match fs::read_to_string("/proc/meminfo") {
        Ok(content) => {
            for line in content.lines().take(10) {
                lines.push(line.trim().to_string());
            }
        }
        Err(_) => lines.push("Cannot read RAM details".to_string()),
    }
    box_print(state, "RAM Details", &lines);
}

fn setup_color(state: &AppState) {
    let names: Vec<String> = colors::NAMES.iter().map(|s| s.to_string()).collect();
    box_print(state, "Available Colors", &names);

    print!("color name: ");
    io::stdout().flush().ok();
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        return;
    }
    let choice = input.trim().to_lowercase();

    if colors::is_valid(&choice) {
        *state.current_color.lock().unwrap() = choice.clone();
        println!("{}", paint(state, &format!("color set to {}", choice)));
    } else {
        println!("{}", paint(state, "invalid color name"));
    }
}

fn show_help(state: &AppState) {
    let commands = [
        "cpu", "ram", "bat", "all", "set", "color", "help", "exit",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect::<Vec<_>>();
    box_print(state, "Commands", &commands);
}

pub fn run(state: Arc<AppState>) {
    println!();
    loop {
        if !state.running.load(Ordering::Relaxed) {
            break;
        }

        print!("_ ");
        io::stdout().flush().ok();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            break;
        }
        let raw = input.trim().to_string();
        let cmd = raw.to_lowercase();

        match cmd.as_str() {
            "exit" => {
                state.running.store(false, Ordering::Relaxed);
                break;
            }
            "cpu" => show_cpu_details(&state),
            "ram" => show_ram_details(&state),
            "bat" => show_bat_details(&state),
            "set" => setup_battery_path(&state),
            "color" => setup_color(&state),
            "all" => {
                show_cpu_details(&state);
                show_ram_details(&state);
                show_bat_details(&state);
            }
            "help" => show_help(&state),
            "" => {}
            _ => println!("{}", paint(&state, &format!("Unknown command: {}", raw))),
        }
    }
}