mod logo;
mod tui;

fn print_help() {
    println!("RecurseVoid v{}", env!("CARGO_PKG_VERSION"));
    println!();
    println!("USAGE:");
    println!("    recursevoid [OPTIONS]");
    println!();
    println!("OPTIONS:");
    println!("    --help     Show this help message");
    println!("    --version  Show version");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "--help" | "-h" => {
                print_help();
                return;
            }
            "--version" | "-V" => {
                println!("RecurseVoid v{}", env!("CARGO_PKG_VERSION"));
                return;
            }
            _ => {}
        }
    }

    tui::start();
}
