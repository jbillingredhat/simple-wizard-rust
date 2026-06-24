use simple_wizard::WizardClient;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: simple-wizard-client <command> [args...]");
        eprintln!("Commands:");
        eprintln!("  set-info --title <title> --description <desc> --help <help>");
        eprintln!("  set-progress --current <n> --total <n> --status <status>");
        eprintln!("  welcome --title <title> --message <message>");
        eprintln!("  file --title <title> --message <message> --default <path>");
        eprintln!("  directory --title <title> --message <message> --default <path>");
        eprintln!("  password --title <title> --message <message> [--no-confirm]");
        eprintln!("  question --title <title> --message <message> --buttons <b1> <b2> ...");
        eprintln!("  text --title <title> --message <message> [--default <text>] [--placeholder <text>] [--validate <preset>]");
        eprintln!("  warning --title <title> --message <message>");
        eprintln!("  error --title <title> --message <message>");
        eprintln!("  complete --title <title> --message <message>");
        eprintln!("  log --message <message>");
        eprintln!("  clear-log");
        eprintln!("  quit");
        std::process::exit(1);
    }

    let client = WizardClient::new("/tmp/simple-wizard.sock");
    let command = &args[1];

    let result = match command.as_str() {
        "set-info" => {
            let title = get_arg(&args, "--title");
            let desc = get_arg(&args, "--description");
            let help = get_arg(&args, "--help");
            client.set_info(title, desc, help)
        }
        "set-progress" => {
            let current = get_arg(&args, "--current").and_then(|s| s.parse().ok());
            let total = get_arg(&args, "--total").and_then(|s| s.parse().ok());
            let status = get_arg(&args, "--status");
            client.set_progress(current, total, status)
        }
        "welcome" => {
            let title = get_arg(&args, "--title").unwrap_or("Welcome");
            let message = get_arg(&args, "--message").unwrap_or("");
            client.show_welcome(title, message)
        }
        "file" => {
            let title = get_arg(&args, "--title").unwrap_or("Select File");
            let message = get_arg(&args, "--message").unwrap_or("");
            let default = get_arg(&args, "--default").unwrap_or("");
            client.show_file(title, message, default)
        }
        "directory" => {
            let title = get_arg(&args, "--title").unwrap_or("Select Directory");
            let message = get_arg(&args, "--message").unwrap_or("");
            let default = get_arg(&args, "--default").unwrap_or("");
            client.show_directory(title, message, default)
        }
        "password" => {
            let title = get_arg(&args, "--title").unwrap_or("Enter Password");
            let message = get_arg(&args, "--message").unwrap_or("");
            let confirm = !args.contains(&"--no-confirm".to_string());
            client.show_password(title, message, confirm)
        }
        "question" => {
            let title = get_arg(&args, "--title").unwrap_or("Question");
            let message = get_arg(&args, "--message").unwrap_or("");
            let buttons = get_args(&args, "--buttons");
            let buttons = if buttons.is_empty() {
                vec!["Yes".to_string(), "No".to_string()]
            } else {
                buttons
            };
            client.show_question(title, message, buttons)
        }
        "text" => {
            let title = get_arg(&args, "--title").unwrap_or("Enter Text");
            let message = get_arg(&args, "--message").unwrap_or("");
            let default = get_arg(&args, "--default").unwrap_or("");
            let placeholder = get_arg(&args, "--placeholder").unwrap_or("");
            let validate = get_arg(&args, "--validate");
            let validation_msg = get_arg(&args, "--validation-message");
            client.show_text(title, message, default, placeholder, validate, validation_msg)
        }
        "warning" => {
            let title = get_arg(&args, "--title").unwrap_or("Warning");
            let message = get_arg(&args, "--message").unwrap_or("");
            client.show_warning(title, message)
        }
        "error" => {
            let title = get_arg(&args, "--title").unwrap_or("Error");
            let message = get_arg(&args, "--message").unwrap_or("");
            client.show_error(title, message)
        }
        "complete" => {
            let title = get_arg(&args, "--title").unwrap_or("Complete");
            let message = get_arg(&args, "--message").unwrap_or("Installation completed successfully!");
            client.show_complete(title, message)
        }
        "log" => {
            let message = get_arg(&args, "--message").unwrap_or("");
            client.append_log(message)
        }
        "clear-log" => client.clear_log(),
        "quit" => client.quit(),
        _ => {
            eprintln!("Unknown command: {}", command);
            std::process::exit(1);
        }
    };

    match result {
        Ok(response) => {
            println!("{}", serde_json::to_string_pretty(&response).unwrap());
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn get_arg<'a>(args: &'a [String], flag: &str) -> Option<&'a str> {
    args.iter()
        .position(|s| s == flag)
        .and_then(|i| args.get(i + 1))
        .map(|s| s.as_str())
}

fn get_args(args: &[String], flag: &str) -> Vec<String> {
    if let Some(start) = args.iter().position(|s| s == flag) {
        args.iter()
            .skip(start + 1)
            .take_while(|s| !s.starts_with("--"))
            .cloned()
            .collect()
    } else {
        Vec::new()
    }
}
