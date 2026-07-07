use std::io::{self, Write};
use std::path::Path;

use crate::logo;
use crate::log::Logger;

fn print_welcome() {
    logo::print_logo();
    println!("RecurseVoid v{} — Type '/help' for commands, '/exit' to quit", env!("CARGO_PKG_VERSION"));
    println!();
}

pub fn start() {
    let log_path = Path::new("recursevoid.log");
    let mut logger = Logger::new(log_path).expect("Failed to open log file");
    logger.log("Session started");

    print_welcome();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        logger.log(input);

        if !input.starts_with('/') {
            println!("Commands must start with '/'. Type '/help' for available commands.");
            continue;
        }

        match input {
            "/help" => {
                println!("Commands:");
                println!("  /help     — Show this message");
                println!("  /version  — Show version");
                println!("  /clear    — Clear screen and reset to welcome");
                println!("  /exit     — Quit RecurseVoid");
            }
            "/version" => {
                println!("RecurseVoid v{}", env!("CARGO_PKG_VERSION"));
            }
            "/clear" => {
                print!("\x1b[2J\x1b[H");
                io::stdout().flush().unwrap();
                print_welcome();
                logger.log("Screen cleared");
            }
            "/exit" | "/quit" => {
                println!("Goodbye.");
                logger.log("Session ended");
                break;
            }
            _ => {
                println!("Unknown command: '{}'. Type '/help' for available commands.", input);
            }
        }
    }
}
