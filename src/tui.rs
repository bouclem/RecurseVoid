use std::io::{self, Write};

use crate::logo;

pub fn start() {
    logo::print_logo();
    println!("RecurseVoid v{} — Type 'help' or 'exit'", env!("CARGO_PKG_VERSION"));
    println!();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        match input {
            "help" => {
                println!("Commands:");
                println!("  help  — Show this message");
                println!("  exit  — Quit RecurseVoid");
            }
            "exit" | "quit" => {
                println!("Goodbye.");
                break;
            }
            "" => continue,
            _ => {
                println!("Unknown command: '{}'. Type 'help' for available commands.", input);
            }
        }
    }
}
