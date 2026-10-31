use std::io::{self, Write};

#[cfg(target_os = "windows")]
pub fn pause_terminal() {
    println!("\nДля выхода из программы нажмите Enter...");
    io::stdout().flush().unwrap();

    let mut discard = String::new();
    let _ = io::stdin().read_line(&mut discard);
}

#[cfg(not(target_os = "windows"))]
fn pause_terminal() {}