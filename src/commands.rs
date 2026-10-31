use std::io::{self, Write};

#[cfg(target_os = "windows")]
pub fn pause_terminal() {
    println!("\nPress Enter to continue");
    io::stdout().flush().unwrap();

    let mut discard = String::new();
    let _ = io::stdin().read_line(&mut discard);
}

#[cfg(not(target_os = "windows"))]
pub fn pause_terminal() {}
