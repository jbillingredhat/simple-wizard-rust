# Simple Wizard - Quick Start Guide

## Installation

### Build from Source
```bash
cargo build --release
```

This creates:
- `target/release/simple-wizard` - The wizard GUI (21MB)
- `target/release/simple-wizard-client` - Command-line client (825KB)

### Install System-Wide (Optional)
```bash
cargo install --path .
```

This installs both binaries to `~/.cargo/bin/`

## Quick Test

Run the quick smoke test:
```bash
./quick_test.sh
```

Run the comprehensive test:
```bash
./test_wizard.sh
```

## Basic Usage

### 1. Start the Wizard GUI

```bash
./target/release/simple-wizard
# Or if installed:
simple-wizard
```

The wizard will start and listen on `/tmp/simple-wizard.sock`

### 2. Control from Another Terminal

```bash
# Set wizard information
./target/release/simple-wizard-client set-info \
  --title "My Installer" \
  --description "Install my application"

# Show a welcome page
./target/release/simple-wizard-client welcome \
  --title "Welcome" \
  --message "Welcome to the installer!\n\nClick Next to continue."

# Show a directory selection
./target/release/simple-wizard-client directory \
  --title "Installation Directory" \
  --default "$HOME/myapp"

# Show text entry with email validation
./target/release/simple-wizard-client text \
  --title "Email Address" \
  --placeholder "user@example.com" \
  --validate email

# Add log messages
./target/release/simple-wizard-client log \
  --message "Installing packages..."

# Show completion
./target/release/simple-wizard-client complete \
  --title "Complete" \
  --message "Installation finished successfully!"

# Quit the wizard
./target/release/simple-wizard-client quit
```

## Using the Rust Client Library

Create a file `my_installer.rs`:

```rust
use simple_wizard::WizardClient;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = WizardClient::new("/tmp/simple-wizard.sock");
    
    // Set wizard info
    client.set_info(
        Some("My App Installer"),
        Some("This will install My App"),
        Some("Follow the prompts"),
    )?;
    
    // Show welcome page
    let response = client.show_welcome(
        "Welcome",
        "Welcome to My App installer!",
    )?;
    
    println!("User clicked: {:?}", response);
    
    // More commands...
    
    client.quit()?;
    Ok(())
}
```

Run it:
```bash
# Terminal 1:
./target/release/simple-wizard

# Terminal 2:
cargo run --release
```

## Example Installer

Try the included example:

```bash
# Terminal 1: Start wizard
./target/release/simple-wizard

# Terminal 2: Run example
cargo run --example example_install
```

## Available Commands

### Wizard Control
- `set-info` - Set wizard title, description, help text
- `set-progress` - Update progress bar and status
- `log` - Add a message to the log panel
- `quit` - Close the wizard

### Page Types
- `welcome` - Welcome/intro page
- `file` - File selection
- `directory` - Directory selection
- `password` - Password entry (with optional confirmation)
- `question` - Question with custom buttons
- `text` - Text entry with validation
- `warning` - Warning message
- `error` - Error message  
- `complete` - Completion/success page

### Validation Presets

Use with the `text` page type via `--validate`:

- `email` - Email addresses
- `url` - HTTP/HTTPS URLs
- `ipv4` - IPv4 addresses
- `port` - Port numbers (1-65535)
- `hostname` - Valid hostnames
- `username` - Usernames (3-32 chars, alphanumeric)
- `number` - Any integer
- `positive_number` - Positive integers
- `alphanumeric` - Letters and numbers only

## Command-Line Help

Get help on any command:
```bash
./target/release/simple-wizard-client --help
./target/release/simple-wizard-client welcome --help
./target/release/simple-wizard-client text --help
```

## Scripting Example

```bash
#!/bin/bash

# Start wizard in background
./target/release/simple-wizard &
WIZARD_PID=$!
sleep 2

# Set up wizard
./target/release/simple-wizard-client set-info \
  --title "Server Setup" \
  --description "Configure your server"

./target/release/simple-wizard-client set-progress \
  --total 3 --current 1 --status "Starting"

# Get hostname
./target/release/simple-wizard-client text \
  --title "Server Hostname" \
  --validate hostname

# Get admin email
./target/release/simple-wizard-client text \
  --title "Admin Email" \
  --validate email

# Get admin password
./target/release/simple-wizard-client password \
  --title "Admin Password" \
  --confirm

# Done
./target/release/simple-wizard-client complete \
  --title "Setup Complete" \
  --message "Server configured successfully!"

./target/release/simple-wizard-client quit
wait $WIZARD_PID
```

## Troubleshooting

### Socket already in use
```bash
rm -f /tmp/simple-wizard.sock
```

### Wizard doesn't start
Check for errors:
```bash
./target/release/simple-wizard 2>&1
```

### Client can't connect
Make sure wizard is running:
```bash
ps aux | grep simple-wizard
ls -la /tmp/simple-wizard.sock
```

## More Information

- See `README.md` for full documentation
- See `COMPLETION_STATUS.md` for technical details
- See `examples/example_install.rs` for a complete example
