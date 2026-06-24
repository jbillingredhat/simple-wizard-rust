# Port Implementation Notes

## Current Status

This is a partial port of the Python/GTK4 `simple-wizard` tool to Rust/Qt5. The project structure is complete with:

✅ **Completed:**
- Project structure and Cargo.toml configuration
- Module organization (lib.rs, pages.rs, wizard.rs, client.rs)
- Client library implementation for controlling the wizard
- Command-line client binary
- Example installation script (Rust version of example_install.py)
- Documentation (README, Makefile)

⚠️ **Blocked:**
- Full Qt5 GUI implementation - System has Qt6 (v6.10.3) but the `qt_widgets` crate v0.5 only supports Qt5

## Qt Version Issue

The current system has Qt6 installed, but the Rust Qt bindings (`qt_widgets`, `qt_core`, `qt_gui`) at version 0.5.0 only support Qt5.

### Options to Resolve:

1. **Install Qt5 alongside Qt6** (Recommended for testing):
   ```bash
   sudo dnf install qt5-qtbase-devel qt5-qtwidgets
   ```
   Then set environment to use Qt5's qmake when building.

2. **Use alternative Qt bindings**:
   - `qmetaobject` - A more modern Rust-Qt binding, but uses a different API paradigm
   - Try newer versions of `qt_widgets` (v0.6.0 exists as `qt_widgets_5`)
   - Consider `cxx-qt` or other modern Qt-Rust bridges

3. **Switch to GTK4 instead of Qt5** (closer to original):
   Since the original is GTK4, could use `gtk4-rs` bindings instead of Qt

4. **Use a pure Rust GUI framework**:
   - `iced` - Modern, elm-inspired GUI framework
   - `egui` - Immediate mode GUI
   - `slint` - Declarative UI framework

## Architecture Overview

The project follows the same client-server architecture as the original Python version:

### Components:

1. **Wizard GUI** (`src/bin/wizard.rs`):
   - Qt5 application window
   - Listens on Unix domain socket (`/tmp/simple-wizard.sock`)
   - Displays wizard pages based on JSON commands

2. **Client Library** (`src/client.rs`):
   - WizardClient struct for programmatic control
   - Methods for each page type (welcome, file, directory, password, etc.)
   - Sends JSON commands over Unix socket

3. **CLI Client** (`src/bin/client.rs`):
   - Command-line interface to the client library
   - Supports all page types and wizard control commands
   - Compatible with bash scripts

4. **Page Types** (`src/pages.rs`):
   - WelcomePage, FilePage, DirectoryPage, PasswordPage
   - QuestionPage, TextEntryPage, WarningPage, ErrorPage, CompletePage
   - Validation presets (email, url, ipv4, port, hostname, etc.)

### IPC Protocol:

Commands sent as JSON:
```json
{
  "command": "show_page",
  "page_type": "welcome",
  "params": {
    "title": "Welcome",
    "message": "Welcome to the installer"
  }
}
```

Responses:
```json
{
  "status": "ok",
  "response": {
    "action": "next"
  }
}
```

## Code Structure

```
simple-wizard-rust/
├── src/
│   ├── lib.rs              # Library root, public API
│   ├── pages.rs            # Page type definitions
│   ├── wizard.rs           # Main wizard window (Qt5)
│   ├── client.rs           # Client library for controlling wizard
│   └── bin/
│       ├── wizard.rs       # Wizard GUI binary
│       └── client.rs       # CLI client binary
├── examples/
│   └── example_install.rs  # Example installer using the library
├── Cargo.toml              # Dependencies and binary configuration
├── README.md               # User documentation
├── Makefile                # Build helpers
└── NOTES.md                # This file
```

## Next Steps

To complete the port:

1. **Resolve Qt version issue** - Choose one of the options above
2. **Implement Qt5 GUI** - Complete the wizard window implementation with actual Qt widgets
3. **Implement page rendering** - Create Qt widgets for each page type
4. **Implement socket server** - Complete the Unix socket server in wizard.rs
5. **Test integration** - Run the example and verify all page types work
6. **Add error handling** - Robust error handling throughout
7. **Add tests** - Unit tests for client library, integration tests

## API Compatibility

The Rust version maintains API compatibility with the Python version:
- Same JSON command structure
- Same page types
- Same validation options
- Same socket path (`/tmp/simple-wizard.sock`)

This means scripts written for the Python version should work with the Rust version without modification.
