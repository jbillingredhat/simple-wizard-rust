use simple_wizard::wizard::run_wizard;

fn main() -> iced::Result {
    let args: Vec<String> = std::env::args().collect();

    // Parse socket path argument
    let socket_path = if let Some(pos) = args.iter().position(|arg| arg == "--socket" || arg == "-s") {
        if pos + 1 < args.len() {
            Some(args[pos + 1].clone())
        } else {
            eprintln!("Error: --socket requires a path argument");
            std::process::exit(1);
        }
    } else {
        None
    };

    // Show help
    if args.iter().any(|arg| arg == "--help" || arg == "-h") {
        println!("Simple Wizard - Interactive installation wizard\n");
        println!("Usage: simple-wizard [OPTIONS]\n");
        println!("Options:");
        println!("  -s, --socket <PATH>   Unix socket path (default: /tmp/simple-wizard.sock)");
        println!("  -h, --help           Show this help message");
        println!("\nExample:");
        println!("  simple-wizard --socket /tmp/my-installer.sock");
        std::process::exit(0);
    }

    // Run the wizard GUI with integrated socket server
    run_wizard(socket_path)
}
