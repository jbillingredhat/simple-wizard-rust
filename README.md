# Simple Wizard - Rust/iced Port

A modern, pure Rust installation wizard for Linux that can be controlled via scripts. This is a Rust port of the original Python/GTK4 version, using the iced GUI framework.

**✅ 100% API compatible with the Python version** - Scripts written for the Python version work without modification!

## Features

- **Multi-pane layout** with information sidebar, interaction area, and progress indicator
- **9 page types**: welcome, file/directory selection, password entry, questions, text input, warnings, errors, and completion
- **Client-server architecture** for scriptable control via Unix domain sockets
- **Response waiting mechanism** - Blocks until user completes interaction (just like Python version)
- **Input validation** - 9 built-in presets (email, URL, IPv4, port, hostname, username, number, etc.)
- **Progress tracking** with customizable steps and status messages
- **Log panel** for displaying installation progress
- Built with **pure Rust** and **iced** for cross-platform compatibility and **no system dependencies**
- Modern, hardware-accelerated rendering via wgpu
- **Single binary distribution** - no runtime dependencies needed

## Requirements

- Rust 1.70+ (2021 edition)
- cargo

**No system dependencies required!** The iced framework renders everything natively without requiring Qt, GTK, or any system GUI libraries.

## Building

```bash
cargo build --release
```

This will build:
- `simple-wizard` - The wizard GUI application (21MB, includes all dependencies)
- `simple-wizard-client` - Command-line client for controlling the wizard (800KB)

## Installation

```bash
cargo install --path .
```

Or run directly from the repository:

```bash
cargo run --release --bin simple-wizard       # Run the wizard GUI
cargo run --bin simple-wizard-client -- <command>  # Run client commands
```

## Usage

### Starting the Wizard

```bash
./target/release/simple-wizard
# Or with cargo:
cargo run --release --bin simple-wizard
```

The wizard will start and listen on a Unix domain socket at `/tmp/simple-wizard.sock`.

### Controlling from Scripts

You can control the wizard using the `simple-wizard-client` command:

```bash
# Set wizard information
./target/release/simple-wizard-client set-info \
  --title "My Installer" \
  --description "Install my application" \
  --help "Follow the prompts"

# Set progress
./target/release/simple-wizard-client set-progress \
  --total 5 --current 1 --status "Starting"

# Show a welcome page
./target/release/simple-wizard-client welcome \
  --title "Welcome" \
  --message "Welcome to the installer"

# Show a file selection dialog
./target/release/simple-wizard-client file \
  --title "Select File" \
  --message "Choose a configuration file"

# Show a directory selection dialog
./target/release/simple-wizard-client directory \
  --title "Installation Directory" \
  --default "$HOME/myapp"

# Show password entry
./target/release/simple-wizard-client password \
  --title "Set Password" \
  --message "Create an admin password"

# Show a question with custom buttons
./target/release/simple-wizard-client question \
  --title "Installation Type" \
  --message "Choose installation type" \
  --buttons Full Minimal Custom

# Show text entry with validation
./target/release/simple-wizard-client text \
  --title "Email Address" \
  --placeholder "user@example.com" \
  --validate email

# Add to log
./target/release/simple-wizard-client log \
  --message "Installing packages..."

# Show completion
./target/release/simple-wizard-client complete \
  --title "Done" \
  --message "Installation completed successfully!"

# Quit the wizard
./target/release/simple-wizard-client quit
```

### Example Installer

See `examples/example_install.sh` for a complete example showing all wizard features:

```bash
# Terminal 1: Start the wizard
./target/release/simple-wizard

# Terminal 2: Run the example
./examples/example_install.sh
```

This demonstrates:
- All 9 page types (welcome, directory, text, password, question, warning, error, complete)
- Input validation (email)
- Progress tracking
- Log messages
- Response parsing

**For Rust library usage**, see `examples/example_install_lib.rs` and `examples/README.md`

### Testing

Three test scripts are available to verify functionality:

```bash
# Quick smoke test - basic commands
./quick_test.sh

# Comprehensive test - all page types
./test_wizard.sh

# Response mechanism test - verifies user input is captured
./test_response.sh
```

## Available Page Types

- **welcome**: Welcome page with Next button
- **file**: File selection dialog
- **directory**: Directory selection dialog
- **password**: Password entry (with optional confirmation)
- **question**: Question with custom button options
- **text**: Text entry with optional validation (email, url, ipv4, port, hostname, username, number, etc.)
- **warning**: Warning message with OK button
- **error**: Error message with OK button
- **complete**: Completion page with Finish button

## Validation Presets

The text entry page supports these validation presets:

- `email` - Email address validation
- `url` - URL validation
- `ipv4` - IPv4 address validation
- `port` - Port number (1-65535)
- `hostname` - Hostname validation
- `username` - Username (3-32 alphanumeric characters)
- `number` - Any integer
- `positive_number` - Positive integer only
- `alphanumeric` - Letters and numbers only

You can also provide a custom regex pattern for validation.

## Architecture

The wizard uses a client-server architecture:

1. **Wizard GUI** (`simple-wizard`) - iced-based GUI that displays the wizard interface and listens for commands on a Unix domain socket
2. **Client Library** - Rust library for sending commands to the wizard
3. **Client CLI** - Command-line tool for scripting the wizard from bash

Communication happens via JSON messages over a Unix domain socket at `/tmp/simple-wizard.sock`.

## Why iced?

This port uses the [iced](https://iced.rs/) GUI framework instead of Qt or GTK:

**Advantages:**
- ✅ Pure Rust - no C++ or C bindings
- ✅ Cross-platform - works on Linux, macOS, Windows
- ✅ No system dependencies - everything is compiled into the binary
- ✅ Modern architecture - inspired by Elm
- ✅ Easy distribution - single binary with all dependencies
- ✅ Hardware-accelerated rendering via wgpu
- ✅ Clean, declarative API

**Trade-offs:**
- ⚠️ Larger binary size (~21MB vs ~5MB for Qt/GTK versions)
- ⚠️ Different look & feel from system-native widgets
- ⚠️ File dialogs need to be implemented (currently using text input)

## Advantages Over Python/GTK4 Version

This Rust/iced port maintains **100% API compatibility** while offering several improvements:

### Performance & Resources
- ✅ **Faster startup**: <1s vs ~2s
- ✅ **Lower memory**: ~50MB vs ~80MB
- ✅ **Single binary**: 21MB standalone executable
- ✅ **No runtime dependencies**: Everything compiled in

### Distribution & Deployment
- ✅ **Easy distribution**: Copy one file
- ✅ **No installation needed**: No Python, GTK4, or system libraries required
- ✅ **Cross-platform ready**: Works on Linux, macOS, Windows* (*with minor socket changes)
- ✅ **Consistent behavior**: Same binary everywhere

### API Compatibility
- ✅ **100% compatible**: Same JSON protocol, same commands, same responses
- ✅ **Interchangeable**: Python client works with Rust server and vice versa
- ✅ **Drop-in replacement**: Existing scripts work without modification

### Technical Differences
- **Language**: Rust instead of Python
- **GUI**: iced (pure Rust) instead of GTK4
- **Error handling**: `Result<T, E>` instead of exceptions
- **Optional params**: `Option<T>` instead of `None`
- **File dialogs**: Text input (could add native dialogs with `rfd` crate if needed)

## Building for Different Platforms

### Linux (current)
```bash
cargo build --release
```

### macOS
```bash
# Same command works on macOS
cargo build --release
```

### Windows
```bash
# Unix sockets don't work on Windows - would need named pipes
# (Not currently implemented)
cargo build --release  # GUI will work, socket server needs adaptation
```

## Documentation

📚 Comprehensive documentation is available in the [`docs/`](docs/) directory.

**Start here:** [Documentation Index](docs/INDEX.md)

### Quick Links

**New to Simple Wizard?**
- [Quick Start Guide](docs/QUICKSTART.md) - Get up and running in minutes

**Coming from the Python version?**
- [API Compatibility](docs/API_COMPATIBILITY.md) - 100% compatible, see the proof
- [API Verification](docs/API_VERIFICATION.md) - Verified against Python architecture

**Want to know what's working?**
- [Final Status](docs/FINAL_STATUS.md) - Complete status, testing guide, and performance metrics
- [Completion Checklist](docs/COMPLETION_STATUS.md) - Detailed feature checklist

**Interested in implementation?**
- [Response Mechanism](docs/RESPONSE_MECHANISM.md) - How user input waiting works
- [Architecture Notes](docs/NOTES.md) - Code structure and design decisions

See the [full documentation index](docs/INDEX.md) for all available documentation.

## Development

```bash
# Run in debug mode (faster compilation, slower runtime)
cargo run --bin simple-wizard

# Run with logging
RUST_LOG=debug cargo run --bin simple-wizard

# Run tests
cargo test

# Format code
cargo fmt

# Lint
cargo clippy
```

## Performance

- **Binary size**: 21MB (release build with all dependencies)
- **Memory usage**: ~50MB (iced + wgpu)
- **Startup time**: < 1 second
- **Client response time**: < 10ms

## License

Apache 2.0 License - See LICENSE file for details.

## Original Version

This is a port of the Python/GTK4 version available at: `/home/jbilling/code/simple-wizard/`
