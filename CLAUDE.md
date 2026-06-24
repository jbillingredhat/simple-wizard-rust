# AI Assistant Guide for simple-wizard-rust

This document provides context for AI assistants working on this project.

## Project Overview

**simple-wizard-rust** is a Rust port of simple-wizard (originally Python/GTK4), providing a simple GUI wizard for shell scripts and installers. It uses the iced GUI framework for hardware-accelerated, cross-platform rendering.

**Key principle**: 100% API compatibility with the Python version. Any shell script that works with Python simple-wizard must work identically with this Rust version.

## Architecture

### Module Structure

```
src/
├── lib.rs                    # Library root, exports public API
├── main.rs                   # Binary entry point, calls run_wizard()
├── wizard/
│   ├── mod.rs               # Re-exports all wizard components
│   ├── types.rs             # Core types (NEVER modify Message/PageType without updating socket.rs)
│   ├── state.rs             # State management, update() returns Task<Message>
│   ├── socket.rs            # Unix socket server (hardcoded to /tmp/simple-wizard.sock)
│   ├── app.rs               # iced application setup and view() method
│   └── ui/
│       ├── mod.rs           # Re-exports panels and pages
│       ├── panels.rs        # Info panel (left), progress panel (bottom)
│       └── pages.rs         # 9 page builders (welcome, file, directory, etc.)
└── pages/
    ├── mod.rs               # Validation utilities
    └── validation.rs        # 9 validation presets with unit tests
```

### Key Design Patterns

**1. State Management**
- All state lives in `WizardWindow` struct
- `update()` method processes `Message` events and returns `Task<Message>`
- Never use bare `return` - always `return Task::none()` or a specific task

**2. Response Waiting**
- Uses `tokio::sync::oneshot` channels for request/response
- Socket handler creates channel, stores sender in shared Arc<Mutex<>>
- User clicks button → response sent through channel → socket returns to client
- 300-second timeout for user responses

**3. Auto-scroll Pattern**
- Log panel has a `scrollable::Id` stored in state
- When log messages added, return `scrollable::scroll_to()` task
- Works for both UI messages AND socket commands

**4. Layout System**
- Use `Length::Fill` for responsive sizing
- Info panel: fixed 250px width
- Content area: fills remaining horizontal space
- Both panels: fill vertical space with `height(Length::Fill)`

## Critical Rules

### DO NOT

1. **Break API compatibility** - Every socket command must work identically to Python version
2. **Use bare returns** - Always return `Task::none()` or a task from `update()`
3. **Modify socket path** - Hardcoded to `/tmp/simple-wizard.sock` for compatibility
4. **Add new Message variants** without updating all match statements
5. **Skip validation tests** - Every validation preset needs unit tests
6. **Create files in root** - Docs go in `docs/`, examples in `examples/`
7. **Write status/completion docs** - Update CHANGELOG.md instead

### DO

1. **Add page types** - Follow pattern in `pages.rs`, add to `PageType` enum
2. **Add validation presets** - Add to `validation.rs` with unit tests
3. **Keep modules focused** - types, state, UI, socket stay separate
4. **Use existing patterns** - Match code style in each file
5. **Update CHANGELOG.md** - Document all changes there
6. **Test manually** - Start wizard, run `examples/example_install.sh`

## Common Tasks

### Adding a New Page Type

1. Add variant to `PageType` enum in `types.rs`
2. Add match arm in `state.rs::process_socket_command()` 
3. Add builder function in `pages.rs::build_page()`
4. Update `CONTRIBUTING.md` with the new type
5. Add example to `examples/example_install.sh`

See `CONTRIBUTING.md` for step-by-step guide.

### Adding a Validation Preset

1. Add function to `pages/validation.rs`
2. Add unit test in same file
3. Add match arm in `state.rs::validate_text()`
4. Update `CONTRIBUTING.md` preset list
5. Run `cargo test` to verify

### Fixing a Bug

1. Identify which module (types, state, socket, UI)
2. Check if it affects API compatibility
3. Add test if it's validation-related
4. Update `CHANGELOG.md` under "Fixed" section
5. Test manually with example script

### Improving UI

1. Layout changes go in `app.rs` or `panels.rs`
2. Page-specific changes go in `pages.rs`
3. Always test that `Length::Fill` still works
4. Verify window resizing still works properly
5. Check auto-scroll still works after changes

## File-Specific Notes

### types.rs
- Defines `Message` enum - every UI action becomes a message
- Defines `PageType` enum - must match Python version exactly
- `WizardWindow` struct - all application state lives here
- **Never change field names** without checking all references

### state.rs
- `update()` signature: `fn update(&mut self, message: Message) -> Task<Message>`
- `process_socket_command()` signature: same return type
- Early returns must use `return Task::none()`
- Validation errors set `self.validation_error` and return early

### socket.rs
- Socket path: `/tmp/simple-wizard.sock` (never change)
- Checks for existing instance before binding
- Removes stale sockets (no active server)
- `show_page` commands wait for response, others return immediately

### app.rs
- `view()` method builds the UI hierarchy
- Main layout: `row![info_panel, content_area]` with `Length::Fill`
- `run_wizard()` sets up socket server in separate thread

### pages.rs
- 9 builder functions, one per page type
- All follow same pattern: title, message, input area, buttons
- Browse buttons use `rfd::FileDialog`
- Password confirmation uses two inputs with match validation

### validation.rs
- 9 presets: email, url, ipv4, port, hostname, username, number, positive_number, alphanumeric
- Every preset has a unit test
- Custom validation messages per preset
- Uses `regex` crate for pattern matching

## Testing

### Unit Tests
```bash
cargo test
```
Runs validation preset tests. All must pass before commit.

### Manual Testing
```bash
# Terminal 1
cargo build --release
./target/release/simple-wizard

# Terminal 2
./examples/example_install.sh
```

Walk through all 9 page types, test Browse buttons, test validation.

### Integration Testing
The example script `examples/example_install.sh` exercises all features:
- All 9 page types
- Progress updates
- Log messages
- Browse buttons (file and directory)
- Validation (email, number presets)
- Success completion

## Building

### Development Build
```bash
cargo build
```
Fast build, includes debug symbols, ~50MB binary.

### Release Build
```bash
cargo build --release
```
Optimized build, ~21MB binary, use this for testing performance.

### Clean Build
```bash
cargo clean && cargo build --release
```
When dependencies change or something seems wrong.

## Dependencies

**Core GUI**: `iced = "0.13"` - Main GUI framework, pure Rust  
**Async Runtime**: `tokio = "1"` - For socket server  
**JSON**: `serde_json = "1"` - Command serialization  
**File Dialogs**: `rfd = "0.14"` - Native OS file pickers  
**Validation**: `regex = "1"` - Pattern matching for presets  

**Do not add dependencies** without good reason. Keep the binary small.

## Code Conventions

### Naming
- Types: `PascalCase` (e.g., `WizardWindow`, `PageType`)
- Functions: `snake_case` (e.g., `build_info_panel`, `validate_text`)
- Constants: `SCREAMING_SNAKE_CASE` (rare in this project)
- Modules: `snake_case` (e.g., `mod wizard`, `mod pages`)

### Organization
- Public API: `lib.rs` exports what external code can use
- Internal: Most items are `pub(crate)` - visible within wizard module
- Private: Helper functions are just `fn`, not `pub`

### Comments
- Module docs: `//!` at top of each file
- Item docs: `///` for public functions/types
- Inline: `//` for clarifying complex logic
- **Don't over-comment** - code should be self-documenting

### Error Handling
- Socket errors: print to stderr and continue
- Validation errors: store in `self.validation_error` and display to user
- JSON parse errors: return error response to client
- **Never panic** - use `Result` or graceful degradation

## API Compatibility Reference

All socket commands from Python version:

```json
{"command": "set_info", "title": "...", "description": "...", "help_text": "..."}
{"command": "set_progress", "current": 2, "total": 5, "status": "..."}
{"command": "append_log", "message": "..."}
{"command": "clear_log"}
{"command": "show_page", "page_type": "...", "params": {...}}
```

Page types: `welcome`, `file`, `directory`, `password`, `question`, `text`, `warning`, `error`, `complete`

Validation presets: `email`, `url`, `ipv4`, `port`, `hostname`, `username`, `number`, `positive_number`, `alphanumeric`

## Performance Expectations

- Binary size: ~21MB (release)
- Startup time: <100ms
- Memory usage: ~15-20MB idle
- Response latency: <10ms for socket commands
- Compile time: ~30 seconds (release build)

If these degrade significantly, investigate before committing.

## Git Workflow

### Commit Messages
Follow conventional commits:
- `feat:` - New feature
- `fix:` - Bug fix
- `docs:` - Documentation changes
- `refactor:` - Code restructuring
- `test:` - Test additions/changes

Always include `Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>` footer.

### Before Committing
1. `cargo build --release` - Must compile clean
2. `cargo test` - All tests must pass
3. Manual test with example script
4. Update `CHANGELOG.md` if user-facing
5. Check `git status` - no unwanted files

## Troubleshooting

### "Socket already in use"
Another wizard instance is running. Kill it with `pkill simple-wizard`.

### UI not responding
Check that `update()` returns `Task`, not `()`. Check for infinite loops.

### Logs not auto-scrolling
Verify `process_socket_command()` returns scroll task for `append_log`.
Verify `log_scroll_id` is set on the scrollable widget.

### Browse buttons not working
Check `rfd` is in `Cargo.toml`. Check `BrowseFile`/`BrowseDirectory` handlers in `state.rs`.

### Validation not working
Check preset name matches exactly in `validate_text()` match statement.
Run `cargo test` to verify preset logic.

## Documentation Structure

- `README.md` - User-facing overview, features, basic usage
- `CONTRIBUTING.md` - How to add page types and validation presets
- `docs/QUICKSTART.md` - Quick start guide for new users
- `docs/IMPLEMENTATION.md` - Technical architecture details
- `docs/CHANGELOG.md` - Project history and milestones
- `docs/INDEX.md` - Documentation navigation
- `docs/NOTES.md` - Development notes
- `examples/README.md` - Example script documentation

Update these when making user-facing or architectural changes.

## Future Enhancements (Ideas for Later)

- Configurable socket path via CLI argument
- Theming support (dark mode)
- i18n/l10n for multiple languages
- More validation presets
- File dialog filters
- Multiple file selection
- Resizable panels
- Log filtering/search

**Don't implement these unless requested.** Focus on maintaining compatibility and stability.

## When in Doubt

1. Check existing code for similar patterns
2. Read `CONTRIBUTING.md` for step-by-step guides
3. Consult `docs/IMPLEMENTATION.md` for architecture
4. Test manually before committing
5. Ask the user if you're unsure about API compatibility

## Remember

This project prioritizes **API compatibility** and **simplicity** over features. When tempted to add something clever, ask: "Does the Python version do this?" If not, probably skip it.

Keep the wizard simple. That's the whole point.
