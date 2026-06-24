# Implementation Guide

This document describes the key implementation details of the Rust/iced port of simple-wizard.

## Architecture Overview

The wizard is structured as a modular iced application with these main components:

### Module Structure

```
src/
├── lib.rs                    # Library root
├── main.rs                   # Binary entry point
├── wizard/
│   ├── mod.rs               # Wizard module root
│   ├── types.rs             # Core types (Message, PageType, WizardWindow)
│   ├── state.rs             # State management and update logic
│   ├── socket.rs            # Unix socket server
│   ├── app.rs               # Application setup and view
│   └── ui/
│       ├── mod.rs           # UI module root
│       ├── panels.rs        # Info and progress panels
│       └── pages.rs         # Page builders (welcome, file, directory, etc.)
└── pages/
    ├── mod.rs               # Page utilities
    └── validation.rs        # Input validation presets
```

### Key Components

#### 1. Socket Server (`socket.rs`)

- Unix domain socket at `/tmp/simple-wizard.sock`
- Accepts JSON commands from clients
- For `show_page` commands, waits for user interaction before responding
- Uses oneshot channels to coordinate response waiting
- Prevents multiple wizard instances on the same socket

**Instance Detection:**
- Checks if socket exists and attempts connection
- If active server found: exits with error
- If stale socket found: removes it and starts normally

#### 2. State Management (`state.rs`)

- `WizardWindow` struct holds all application state
- `update()` method processes `Message` events and returns `Task<Message>`
- `process_socket_command()` handles incoming socket commands
- Input validation using presets from `pages/validation.rs`

**Response Waiting Mechanism:**
- Uses `tokio::sync::oneshot` channels
- Socket handler creates channel, stores sender in shared state
- When user clicks Next/Cancel, sends response through channel
- Socket waits up to 300 seconds for response

#### 3. UI System (`ui/`)

**Panels** (`panels.rs`):
- Info panel: title, description, help text (left side, fixed 250px width)
- Progress panel: progress bar, status text, scrollable log (bottom)
- Auto-scroll: log panel scrolls to bottom on new messages

**Pages** (`pages.rs`):
Nine page types with consistent interface:
- `welcome` - Welcome/intro page
- `file` - File selection with browse button
- `directory` - Directory selection with browse button  
- `password` - Password entry with optional confirmation
- `question` - Multiple choice buttons
- `text` - Free text input with validation
- `warning` - Warning message
- `error` - Error message
- `complete` - Success/completion message

#### 4. Validation System (`pages/validation.rs`)

Nine built-in validation presets:
- `email` - Email address format
- `url` - HTTP/HTTPS URL format
- `ipv4` - IPv4 address format
- `port` - Port number (1-65535)
- `hostname` - Valid hostname format
- `username` - Alphanumeric + underscore/hyphen
- `number` - Any number (integer or float)
- `positive_number` - Positive numbers only
- `alphanumeric` - Letters and numbers only

## Key Features

### Auto-Scroll Log Panel

The log panel automatically scrolls to the bottom when new messages are added:

1. `log_scroll_id` field in `WizardWindow` for programmatic scrolling
2. `update()` returns `Task<Message>` instead of `()`
3. `AppendLog` message returns scroll task
4. Socket `append_log` command also returns scroll task
5. Uses `scrollable::scroll_to()` with `AbsoluteOffset { y: f32::MAX }`

### Native File Dialogs

Uses the `rfd` crate for cross-platform native file dialogs:

```rust
Message::BrowseFile => {
    if let Some(file) = rfd::FileDialog::new().pick_file() {
        self.file_path = file.to_string_lossy().to_string();
    }
}

Message::BrowseDirectory => {
    if let Some(dir) = rfd::FileDialog::new().pick_folder() {
        self.file_path = dir.to_string_lossy().to_string();
    }
}
```

### Responsive UI Layout

All panels use `Length::Fill` for proper resizing:

```rust
// Main content expands to fill window
let main_content = row![
    container(info_panel).width(250).height(Length::Fill),
    container(content_area).width(Length::Fill).height(Length::Fill),
].height(Length::Fill);

// Progress bar and log panel span full width
progress_bar(0.0..=1.0, progress).width(Length::Fill)
scrollable(log_text).width(Length::Fill).height(150)
```

## API Compatibility

100% compatible with the Python/GTK4 version. All socket commands work identically:

### Commands

**set_info** - Update info panel
```json
{"command": "set_info", "title": "...", "description": "...", "help_text": "..."}
```

**set_progress** - Update progress bar
```json
{"command": "set_progress", "current": 2, "total": 5, "status": "Installing..."}
```

**append_log** - Add log message
```json
{"command": "append_log", "message": "Step completed"}
```

**clear_log** - Clear all log messages
```json
{"command": "clear_log"}
```

**show_page** - Display page and wait for user input
```json
{
  "command": "show_page",
  "page_type": "file",
  "params": {
    "title": "Select File",
    "message": "Choose a file to process",
    "default": "/home/user"
  }
}
```

## Adding New Features

### Adding a New Page Type

See `CONTRIBUTING.md` for step-by-step instructions.

### Adding a New Validation Preset

See `CONTRIBUTING.md` for step-by-step instructions.

### Custom Socket Path

Currently hardcoded to `/tmp/simple-wizard.sock`. To make configurable:

1. Add command-line argument parsing
2. Pass socket path to `run_socket_server()`
3. Update client to accept `--socket` argument

## Testing

### Manual Testing

```bash
# Terminal 1: Start wizard
./target/release/simple-wizard

# Terminal 2: Run example
./examples/example_install.sh
```

### Test Coverage

The validation system has comprehensive unit tests:

```bash
cargo test
```

## Performance Notes

- Binary size: ~21MB (release build with optimizations)
- Startup time: <100ms
- Memory usage: ~15-20MB idle
- Response latency: <10ms for socket commands

The iced framework provides hardware-accelerated rendering for smooth UI performance.
