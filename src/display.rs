use crate::app_state::AppState;
use crate::colors::paint;
use libc::{ioctl, winsize, STDOUT_FILENO, TIOCGWINSZ};

fn terminal_columns() -> usize {
    unsafe {
        let mut ws: winsize = std::mem::zeroed();
        if ioctl(STDOUT_FILENO, TIOCGWINSZ, &mut ws) == 0 && ws.ws_col != 0 {
            ws.ws_col as usize
        } else {
            60
        }
    }
}

pub fn box_print(state: &AppState, title: &str, lines: &[String]) {
    let mut width = title.len();
    for l in lines {
        width = width.max(l.len());
    }
    width += 4;
    width = width.max(terminal_columns() / 2);

    let top = format!("+{}+", "-".repeat(width - 2));
    println!("{}", paint(state, &top));
    println!("{}", paint(state, &format!("| {} |", pad(title, width - 4))));
    println!("{}", paint(state, &top));
    for l in lines {
        println!("{}", paint(state, &format!("| {} |", pad(l, width - 4))));
    }
    println!("{}", paint(state, &top));
}

fn pad(text: &str, width: usize) -> String {
    if text.len() >= width {
        text.to_string()
    } else {
        format!("{}{}", text, " ".repeat(width - text.len()))
    }
}