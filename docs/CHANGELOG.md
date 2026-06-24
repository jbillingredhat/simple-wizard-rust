# Changelog

All notable changes to the Rust/iced port of simple-wizard.

## [Unreleased]

### Added
- Initial Rust/iced port from Python/GTK4
- Modular architecture with separate modules for types, state, UI, and socket
- Nine page types: welcome, file, directory, password, question, text, warning, error, complete
- Nine validation presets: email, url, ipv4, port, hostname, username, number, positive_number, alphanumeric
- Native file dialogs using rfd crate (cross-platform)
- Auto-scrolling log panel
- Socket instance detection (prevents multiple wizards on same socket)
- Comprehensive documentation and examples

### Changed
- Ported from Python/GTK4 to Rust/iced
- Replaced GTK4 file dialogs with rfd native dialogs
- Modularized monolithic files into logical components

### Fixed
- Response waiting mechanism now properly waits for user input
- Browse buttons now open native OS file/directory dialogs
- UI now fills entire window and responds to resizing
- Log panel now auto-scrolls to bottom on new messages
- Socket file conflict detection prevents silent overwrite

## Implementation Milestones

### Phase 1: Core Port
- ✅ Basic iced application structure
- ✅ Unix socket server with JSON protocol
- ✅ All nine page types implemented
- ✅ API compatibility with Python version verified

### Phase 2: Response Mechanism
- ✅ Oneshot channels for response waiting
- ✅ Socket handler waits for user interaction
- ✅ 300-second timeout for responses

### Phase 3: Modularization
- ✅ Split wizard.rs into types, state, app, socket
- ✅ Organized UI components into panels and pages
- ✅ Extracted validation into separate module with tests

### Phase 4: Feature Parity
- ✅ Native file dialogs with rfd
- ✅ Responsive UI layout with Length::Fill
- ✅ Auto-scrolling log panel
- ✅ Socket instance detection

### Phase 5: Polish
- ✅ Documentation organization
- ✅ Shell script example replacing Rust example
- ✅ Contributing guide for extensibility

## API Compatibility

The Rust port maintains 100% API compatibility with the Python version:

| Feature | Python/GTK4 | Rust/iced | Status |
|---------|-------------|-----------|--------|
| Unix socket | ✅ | ✅ | ✅ Compatible |
| JSON protocol | ✅ | ✅ | ✅ Compatible |
| All 9 page types | ✅ | ✅ | ✅ Compatible |
| Info panel | ✅ | ✅ | ✅ Compatible |
| Progress tracking | ✅ | ✅ | ✅ Compatible |
| Log messages | ✅ | ✅ | ✅ Compatible |
| File dialogs | ✅ | ✅ | ✅ Compatible |
| Input validation | ✅ | ✅ | ✅ Compatible |
| Response waiting | ✅ | ✅ | ✅ Compatible |

## Performance

- **Binary size**: ~21MB (release build)
- **Startup time**: <100ms
- **Memory usage**: ~15-20MB idle
- **Compile time**: ~30 seconds (release build)

## Dependencies

Major dependencies:
- `iced = "0.13"` - GUI framework
- `tokio = "1"` - Async runtime
- `serde_json = "1"` - JSON serialization
- `rfd = "0.14"` - Native file dialogs
- `regex = "1"` - Input validation

## Future Enhancements

Potential improvements:
- [ ] Configurable socket path via CLI argument
- [ ] Theming support (dark mode, custom colors)
- [ ] i18n/l10n for multiple languages
- [ ] Additional validation presets
- [ ] File dialog filters (file type restrictions)
- [ ] Multiple file selection support
- [ ] Resizable panels
- [ ] Log filtering and search
