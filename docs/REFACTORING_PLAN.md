# Wizard.rs Refactoring Plan

## Current State

`src/wizard.rs` is **831 lines** with multiple distinct responsibilities mixed together.

## Analysis

### Logical Sections Identified

1. **Types & Messages** (~60 lines)
   - `Message` enum (30 lines)
   - `PageType` enum (15 lines)
   - `WizardWindow` struct (20 lines)
   - `CurrentPage` struct (15 lines)

2. **UI Rendering** (~300 lines)
   - 9 page builder functions (`build_*_page`)
   - Panel builders (`build_info_panel`, `build_progress_panel`)
   - Main view composition
   - ~30-40 lines per page builder

3. **State Management** (~150 lines)
   - `update()` function
   - `show_page()` function
   - `process_socket_command()` function
   - Response building and sending

4. **Socket Server** (~140 lines)
   - `run_socket_server()` async function
   - `handle_connection()` async function
   - Connection management

5. **Application Setup** (~80 lines)
   - `run_wizard()` function
   - Channel setup
   - Subscription configuration

## Proposed Modular Structure

```
src/wizard/
├── mod.rs              # Public API, re-exports
├── types.rs            # Message, PageType, WizardWindow, CurrentPage
├── state.rs            # State management (update, process_command)
├── ui/                 # UI rendering (could split further)
│   ├── mod.rs          # UI module exports
│   ├── pages.rs        # All page builders (or split into page/*.rs)
│   └── panels.rs       # Info panel, progress panel
├── socket.rs           # Socket server and handlers
└── app.rs              # Application setup (run_wizard)
```

### Alternative: Even More Granular

```
src/wizard/
├── mod.rs              # Public API
├── types.rs            # Core types
├── state.rs            # State management
├── ui/
│   ├── mod.rs          # UI exports
│   ├── panels.rs       # Info and progress panels
│   └── pages/          # One file per page type
│       ├── mod.rs      # Page UI exports
│       ├── welcome.rs  # Welcome page builder
│       ├── file.rs     # File page builder
│       ├── password.rs # Password page builder
│       ├── question.rs # Question page builder
│       ├── text.rs     # Text entry page builder
│       └── ...
├── socket.rs           # Socket server
└── app.rs              # App runner

```

## Recommended Approach

**Option 1: Moderate Split** (Recommended first step)
- Split into 5-6 files
- Each file is 100-200 lines
- Clear separation of concerns
- Easy to navigate

**Option 2: Granular Split** (Future enhancement)
- Each page type in its own file
- Maximum modularity
- Easiest to extend
- More files to manage

## Benefits

### For Maintainability
✅ **Easier to navigate** - Find code by responsibility  
✅ **Easier to understand** - Each file has one purpose  
✅ **Easier to modify** - Changes are localized  
✅ **Easier to test** - Can test modules independently  

### For Contributors
✅ **Easier to add pages** - Just add to ui/pages.rs (or new file)  
✅ **Easier to modify UI** - UI code isolated from logic  
✅ **Easier to extend socket** - Socket code in one place  
✅ **Clear structure** - Know where everything is  

### For Code Quality
✅ **Separation of concerns** - UI, state, network separate  
✅ **Smaller files** - Each file is manageable  
✅ **Better organization** - Logical grouping  
✅ **Future-proof** - Easy to add features  

## Proposed File Breakdown

### 1. `src/wizard/types.rs` (~80 lines)
```rust
// Message enum
// PageType enum
// WizardWindow struct
// CurrentPage struct
```

### 2. `src/wizard/state.rs` (~150 lines)
```rust
impl WizardWindow {
    pub fn new(...) -> Self { ... }
    pub fn update(&mut self, message: Message) { ... }
    pub fn subscription(&self) -> Subscription<Message> { ... }
    
    fn process_socket_command(&mut self, cmd: Value) { ... }
    fn show_page(&mut self, page_type: PageType, params: Value) { ... }
    fn validate_text(...) -> Result<(), String> { ... }
    fn build_response(...) -> Value { ... }
    fn send_response(...) { ... }
}
```

### 3. `src/wizard/ui/mod.rs` (~20 lines)
```rust
// Re-exports
pub use self::pages::*;
pub use self::panels::*;

mod pages;
mod panels;
```

### 4. `src/wizard/ui/panels.rs` (~80 lines)
```rust
impl WizardWindow {
    pub fn build_info_panel(&self) -> Column<'_, Message> { ... }
    pub fn build_progress_panel(&self) -> Column<'_, Message> { ... }
}
```

### 5. `src/wizard/ui/pages.rs` (~300 lines)
```rust
impl WizardWindow {
    pub fn build_welcome_page(...) -> Element<...> { ... }
    pub fn build_file_page(...) -> Element<...> { ... }
    pub fn build_directory_page(...) -> Element<...> { ... }
    pub fn build_password_page(...) -> Element<...> { ... }
    pub fn build_question_page(...) -> Element<...> { ... }
    pub fn build_text_page(...) -> Element<...> { ... }
    pub fn build_warning_page(...) -> Element<...> { ... }
    pub fn build_error_page(...) -> Element<...> { ... }
    pub fn build_complete_page(...) -> Element<...> { ... }
    pub fn build_content_area(&self) -> Element<...> { ... }
}
```

### 6. `src/wizard/socket.rs` (~140 lines)
```rust
pub async fn run_socket_server(...) { ... }
async fn handle_connection(...) { ... }
```

### 7. `src/wizard/app.rs` (~80 lines)
```rust
pub fn run_wizard() -> iced::Result { ... }
```

### 8. `src/wizard/mod.rs` (~30 lines)
```rust
// Module declarations
mod types;
mod state;
mod ui;
mod socket;
mod app;

// Re-exports
pub use types::*;
pub use app::run_wizard;
```

## Migration Path

### Phase 1: Basic Split (This PR)
1. Create `src/wizard/` directory
2. Create core modules (types, state, ui, socket, app)
3. Move code to appropriate modules
4. Update imports in lib.rs
5. Verify builds and tests pass
6. Update CONTRIBUTING.md

### Phase 2: UI Refinement (Future PR)
1. Split ui/pages.rs into individual page files
2. Create ui/pages/ directory
3. One file per page type
4. Update CONTRIBUTING.md with new pattern

### Phase 3: Further Improvements (Future PRs)
1. Add traits for page types
2. Add builder patterns
3. Add more tests
4. Add benchmarks

## Implementation Checklist

- [ ] Create `src/wizard/` directory
- [ ] Create `types.rs` with enums and structs
- [ ] Create `state.rs` with state management
- [ ] Create `ui/mod.rs` with UI module structure
- [ ] Create `ui/panels.rs` with panel builders
- [ ] Create `ui/pages.rs` with page builders
- [ ] Create `socket.rs` with socket server
- [ ] Create `app.rs` with run_wizard
- [ ] Create `mod.rs` with re-exports
- [ ] Update `src/lib.rs` imports
- [ ] Verify `cargo build` works
- [ ] Verify `cargo test` passes
- [ ] Verify `cargo clippy` is clean
- [ ] Update `CONTRIBUTING.md`
- [ ] Test all functionality manually
- [ ] Commit changes

## Risks & Mitigation

### Risk: Breaking Changes
**Mitigation**: Re-export everything from `mod.rs` to maintain public API

### Risk: Import Confusion
**Mitigation**: Clear re-export structure, update docs

### Risk: Merge Conflicts
**Mitigation**: Do this as a standalone PR, don't mix with features

### Risk: Harder to Find Code
**Mitigation**: Clear module organization, update CONTRIBUTING.md

## Testing Strategy

1. **Build Test**: `cargo build --release`
2. **Unit Tests**: `cargo test`
3. **Clippy**: `cargo clippy`
4. **Manual Test**: Run all 3 test scripts
5. **Example Test**: Run example installer
6. **Smoke Test**: Verify each page type works

## Documentation Updates

- [ ] Update CONTRIBUTING.md with new structure
- [ ] Add module-level docs to each file
- [ ] Update README if needed
- [ ] Create REFACTORING_PART2.md summary

## Estimated Effort

- **Phase 1 (Basic Split)**: 30-45 minutes
- **Testing & Verification**: 15-20 minutes
- **Documentation Updates**: 15-20 minutes
- **Total**: ~1-1.5 hours

## Decision

**Recommendation: Proceed with Phase 1 (Basic Split)**

Benefits outweigh risks:
- ✅ Clear improvement in organization
- ✅ Low risk (re-exports maintain API)
- ✅ Foundation for future improvements
- ✅ Easier for contributors

The modular structure will make the codebase much more maintainable and easier to extend.

**Ready to proceed?** 🚀
