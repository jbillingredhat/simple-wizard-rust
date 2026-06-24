# Native File Dialogs - Implementation Plan

## Current Status

The file and directory pages currently have "Browse..." buttons that don't do anything. They're just placeholders showing text input only.

## Issue

The Browse buttons for file/directory selection are non-functional:
- `Message::BrowseFile` is defined but not handled
- `Message::BrowseDirectory` is defined but not handled
- Users can only type paths manually

## Solutions

### Option 1: Add Native File Dialogs (Recommended)

Use the `rfd` (Rust File Dialog) crate to show native file/directory pickers.

**Pros:**
- ✅ Native OS dialogs (looks professional)
- ✅ User-friendly (browse file system visually)
- ✅ Cross-platform (Windows, macOS, Linux)
- ✅ Actively maintained crate

**Cons:**
- ⚠️ Adds dependency (~100KB)
- ⚠️ Async file dialog needs careful integration with iced

**Implementation:**

1. Add to `Cargo.toml`:
```toml
[dependencies]
rfd = "0.14"
```

2. Update `wizard/state.rs` to handle browse messages:
```rust
Message::BrowseFile => {
    // Launch file picker
    if let Some(path) = rfd::FileDialog::new().pick_file() {
        self.file_path = path.to_string_lossy().to_string();
    }
}
Message::BrowseDirectory => {
    // Launch directory picker
    if let Some(path) = rfd::FileDialog::new().pick_folder() {
        self.file_path = path.to_string_lossy().to_string();
    }
}
```

3. Handle async properly with iced Tasks if needed

**Estimated effort:** 30-60 minutes

### Option 2: Remove Browse Buttons

Simply remove the non-functional buttons and rely on text input only.

**Pros:**
- ✅ No additional dependencies
- ✅ Simple, clear UX
- ✅ Works everywhere
- ✅ 5 minute fix

**Cons:**
- ⚠️ Less user-friendly (must type paths)
- ⚠️ Doesn't match Python version

**Implementation:**

Just remove the Browse buttons from `wizard/ui/pages.rs`:
```rust
// Remove this line:
button(text("Browse...")).on_press(Message::BrowseFile),
```

**Estimated effort:** 5 minutes

### Option 3: Implement Tab Completion (Advanced)

Add path auto-completion when user types.

**Pros:**
- ✅ No file dialog needed
- ✅ Unix-like experience
- ✅ Keyboard-friendly

**Cons:**
- ⚠️ Complex to implement
- ⚠️ Not standard for wizards
- ⚠️ Requires filesystem access logic

**Estimated effort:** 2-3 hours

## Recommendation

**Implement Option 1 (Native File Dialogs) OR Option 2 (Remove Buttons)**

### If you want feature parity with Python:
→ **Option 1** - Add `rfd` crate for native dialogs

### If you want to keep dependencies minimal:
→ **Option 2** - Remove browse buttons, document that users type paths

## Quick Fix: Option 2 Implementation

For now, let's remove the non-functional buttons to avoid user confusion:

### Changes needed:

1. **Remove Browse buttons** from `src/wizard/ui/pages.rs`
2. **Remove BrowseFile/BrowseDirectory messages** from `src/wizard/types.rs`
3. **Update documentation** to clarify text-input-only

Or add a note that Browse is not yet implemented.

## Future Enhancement: Option 1 Implementation

If we decide to add native file dialogs later:

### Step 1: Add dependency
```toml
[dependencies]
rfd = "0.14"
```

### Step 2: Handle in update()
```rust
Message::BrowseFile => {
    use rfd::FileDialog;
    if let Some(file) = FileDialog::new().pick_file() {
        self.file_path = file.to_string_lossy().to_string();
    }
}
Message::BrowseDirectory => {
    use rfd::FileDialog;
    if let Some(dir) = FileDialog::new().pick_folder() {
        self.file_path = dir.to_string_lossy().to_string();
    }
}
```

### Step 3: Handle async if needed
`rfd` supports both blocking and async. For iced, we might need async:
```rust
Message::BrowseFile => {
    // Spawn async file dialog
    return iced::Task::perform(
        async {
            rfd::AsyncFileDialog::new()
                .pick_file()
                .await
                .map(|f| f.path().to_path_buf())
        },
        |path| {
            if let Some(p) = path {
                Message::FilePathChanged(p.to_string_lossy().to_string())
            } else {
                Message::Nothing  // User cancelled
            }
        }
    );
}
```

## Decision

**For this PR:** Implement a quick fix to avoid user confusion

**For future:** Consider adding `rfd` for full feature parity

## Compatibility Note

The Python version uses GTK4's native file dialogs:
```python
dialog = Gtk.FileDialog()
dialog.select_folder(callback=self._on_folder_selected)
```

The Rust equivalent would be `rfd::FileDialog`. This maintains API compatibility at the protocol level - the client doesn't care how the path is selected, just that it receives a path back.

## Testing

After implementing either option:

1. **Manual test:**
   ```bash
   ./target/release/simple-wizard
   ./target/release/simple-wizard-client directory --title "Test" --default "/tmp"
   ```

2. **Verify:**
   - Can type paths manually ✓
   - Browse button works (Option 1) OR is removed (Option 2) ✓
   - Path is captured correctly ✓
   - Response sent back to client ✓

## References

- rfd crate: https://crates.io/crates/rfd
- iced async: https://docs.rs/iced/latest/iced/trait.Task.html
- Python implementation: /home/jbilling/code/simple-wizard/simple_wizard/pages.py
