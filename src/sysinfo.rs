use std::fs;

const THERMAL_PATH: &str = "/sys/class/thermal/thermal_zone0/temp";

pub fn get_cpu_temp() -> String {
    match fs::read_to_string(THERMAL_PATH) {
        Ok(content) => match content.trim().parse::<f64>() {
            Ok(value) => format!("{:.1} C", value / 1000.0),
            Err(_) => "N/A".to_string(),
        },
        Err(_) => "N/A".to_string(),
    }
}

pub fn get_ram() -> String {
    let content = match fs::read_to_string("/proc/meminfo") {
        Ok(c) => c,
        Err(_) => return "N/A".to_string(),
    };

    let mut mem_total: u64 = 0;
    let mut mem_free: u64 = 0;
    let mut mem_avail: u64 = 0;

    for line in content.lines() {
        if let Some(value) = extract_kb_value(line, "MemTotal") {
            mem_total = value;
        } else if let Some(value) = extract_kb_value(line, "MemFree") {
            mem_free = value;
        } else if let Some(value) = extract_kb_value(line, "MemAvailable") {
            mem_avail = value;
        }
    }

    let used = if mem_avail != 0 {
        mem_total.saturating_sub(mem_avail)
    } else {
        mem_total.saturating_sub(mem_free)
    };

    let used_gb = used as f64 / 1024.0 / 1024.0;
    let total_gb = mem_total as f64 / 1024.0 / 1024.0;
    format!("{:.1}/{:.1} GB", used_gb, total_gb)
}

fn extract_kb_value(line: &str, key: &str) -> Option<u64> {
    if !line.starts_with(key) {
        return None;
    }
    line.split_whitespace().nth(1)?.parse().ok()
}