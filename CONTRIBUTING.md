# Contributing to Simple Wizard

Thank you for your interest in contributing to Simple Wizard! This guide will help you understand the project structure and how to add new features.

## Table of Contents

- [Project Structure](#project-structure)
- [Adding New Page Types](#adding-new-page-types)
- [Adding New Validation Presets](#adding-new-validation-presets)
- [Code Style](#code-style)
- [Testing](#testing)
- [Documentation](#documentation)
- [Submitting Changes](#submitting-changes)

## Project Structure

```
src/
├── lib.rs              # Library exports
├── pages/              # Page system (modular!)
│   ├── mod.rs          # Page module exports
│   └── validation.rs   # Validation presets
├── wizard.rs           # GUI and socket server
├── client.rs           # Client library
└── bin/
    ├── wizard.rs       # GUI binary
    └── client.rs       # CLI client
```

## Adding New Page Types

The wizard currently supports 9 page types. To add a new page type, follow these steps:

### 1. Understand the Current System

Currently, page types are defined as an enum in `src/wizard.rs`:

```rust
pub enum PageType {
    Welcome,
    File,
    Directory,
    Password,
    Question,
    Text,
    Warning,
    Error,
    Complete,
}
```

### 2. Add Your Page Type

**Step 1**: Add the new variant to `PageType` enum in `src/wizard.rs`:

```rust
pub enum PageType {
    Welcome,
    File,
    Directory,
    Password,
    Question,
    Text,
    Warning,
    Error,
    Complete,
    YourNewType,  // Add here
}
```

**Step 2**: Add the page type to the match in `process_socket_command()`:

```rust
let page_type = match page_type_str {
    "welcome" => PageType::Welcome,
    "file" => PageType::File,
    // ... existing types ...
    "your_new_type" => PageType::YourNewType,  // Add here
    _ => return,
};
```

**Step 3**: Add rendering logic in `build_content_area()`:

```rust
match page.page_type {
    PageType::Welcome => self.build_welcome_page(page),
    // ... existing types ...
    PageType::YourNewType => self.build_your_new_page(page),  // Add here
}
```

**Step 4**: Implement the builder method:

```rust
fn build_your_new_page<'a>(&'a self, page: &'a CurrentPage) -> Element<'a, Message> {
    column![
        text(&page.title).size(20),
        text(&page.message).size(14),
        // Your custom UI elements here
        button(text("Next")).on_press(Message::NextClicked),
    ]
    .spacing(12)
    .padding(24)
    .align_x(Alignment::Center)
    .into()
}
```

### 3. Add Client Library Method

Add a method to `WizardClient` in `src/client.rs`:

```rust
pub fn show_your_new_type(&self, title: &str, message: &str, custom_param: &str) -> Result<Response, String> {
    let mut page_params = HashMap::new();
    page_params.insert("title".to_string(), serde_json::Value::String(title.to_string()));
    page_params.insert("message".to_string(), serde_json::Value::String(message.to_string()));
    page_params.insert("custom_param".to_string(), serde_json::Value::String(custom_param.to_string()));

    let mut params = HashMap::new();
    params.insert("page_type".to_string(), serde_json::Value::String("your_new_type".to_string()));
    params.insert("params".to_string(), serde_json::Value::Object(
        page_params.into_iter().map(|(k, v)| (k, v)).collect()
    ));

    self.send_command(Command {
        command: "show_page".to_string(),
        params,
    })
}
```

### 4. Add CLI Command

Add a subcommand to the CLI in `src/bin/client.rs`:

```rust
// In the subparsers section:
let your_parser = subparsers.add_parser("your-new-type", help="Show your new type page");
your_parser.add_argument("--title", default="Your Title", help="Title");
your_parser.add_argument("--message", default="", help="Message");
your_parser.add_argument("--custom-param", default="", help="Custom parameter");

// In the match statement:
Some("your-new-type") => {
    let response = client.show_your_new_type(
        &args.title,
        &args.message,
        &args.custom_param,
    )?;
    println!("{}", serde_json::to_string_pretty(&response)?);
}
```

### 5. Update Documentation

Update these files:
- `README.md` - Add to the page types list
- `docs/API_COMPATIBILITY.md` - Document the new page type
- `docs/QUICKSTART.md` - Add usage example
- This file (CONTRIBUTING.md) - Add to examples if it's a good template

### 6. Add Tests

Add a test case to `test_wizard.sh`:

```bash
echo "X. Testing your new type..."
./target/release/simple-wizard-client your-new-type \
  --title "Test Title" \
  --message "Test Message" \
  --custom-param "value"

sleep 2
```

### Example: Adding a "Progress" Page

Here's a complete example of adding a progress page that shows a spinner:

**1. Add to PageType enum:**
```rust
pub enum PageType {
    // ... existing types ...
    Progress,
}
```

**2. Add to command processing:**
```rust
"progress" => PageType::Progress,
```

**3. Add builder:**
```rust
fn build_progress_page<'a>(&'a self, page: &'a CurrentPage) -> Element<'a, Message> {
    column![
        text(&page.title).size(20),
        text(&page.message).size(14),
        text("⏳").size(48),  // Spinner icon
        text("Please wait...").size(12),
    ]
    .spacing(12)
    .padding(24)
    .align_x(Alignment::Center)
    .into()
}
```

**4. Add to content builder:**
```rust
PageType::Progress => self.build_progress_page(page),
```

**5. Add client method:**
```rust
pub fn show_progress(&self, title: &str, message: &str) -> Result<Response, String> {
    // Similar to other show_* methods
}
```

**6. Add CLI command:**
```bash
simple-wizard-client progress --title "Installing" --message "Installing packages..."
```

## Adding New Validation Presets

Validation presets are defined in `src/pages/validation.rs`.

### Steps

**1. Add to the match statement in `ValidationPresets::get()`:**

```rust
pub fn get(preset: &str) -> Option<(&'static str, &'static str)> {
    match preset {
        // ... existing presets ...
        "your_preset" => Some((
            r"^your-regex-pattern$",
            "Your user-friendly error message",
        )),
        _ => None,
    }
}
```

**2. Add to `available_presets()`:**

```rust
pub fn available_presets() -> Vec<&'static str> {
    vec![
        // ... existing presets ...
        "your_preset",
    ]
}
```

**3. Add tests:**

```rust
#[test]
fn test_your_preset() {
    assert!(ValidationPresets::exists("your_preset"));
    let (pattern, _) = ValidationPresets::get("your_preset").unwrap();
    let re = regex::Regex::new(pattern).unwrap();
    assert!(re.is_match("valid_example"));
    assert!(!re.is_match("invalid_example"));
}
```

**4. Update documentation:**
- `README.md` - Add to validation presets list
- `docs/API_COMPATIBILITY.md` - Add to presets table
- `docs/QUICKSTART.md` - Add usage example

### Example: Adding a "phone" Preset

```rust
"phone" => Some((
    r"^\+?1?\d{10,14}$",
    "Please enter a valid phone number (e.g., +1234567890)",
)),
```

## Code Style

### Rust Code

- **Follow Rust conventions**: Use `cargo fmt` before committing
- **Lint your code**: Run `cargo clippy` and fix warnings
- **Write docs**: Add doc comments for public functions and types
- **Use meaningful names**: Prefer clarity over brevity

### Example Good Style

```rust
/// Shows a warning page to the user
///
/// # Arguments
///
/// * `title` - The warning title
/// * `message` - The warning message
///
/// # Returns
///
/// The user's response (typically "ok")
pub fn show_warning(&self, title: &str, message: &str) -> Result<Response, String> {
    // Implementation
}
```

### Commit Messages

Follow conventional commits:

```
feat: Add progress page type
fix: Correct email validation regex
docs: Update QUICKSTART with new examples
test: Add validation preset tests
refactor: Modularize page system
```

## Testing

### Manual Testing

```bash
# Build
cargo build --release

# Run quick test
./quick_test.sh

# Run comprehensive test
./test_wizard.sh

# Run response test
./test_response.sh
```

### Automated Testing

```bash
# Run unit tests
cargo test

# Run with verbose output
cargo test -- --nocapture

# Run specific test
cargo test test_email_preset
```

### Integration Testing

Test the full flow:
1. Start the wizard: `./target/release/simple-wizard`
2. Run your commands: `./target/release/simple-wizard-client ...`
3. Verify behavior matches expectations

## Documentation

### What to Document

When adding features, update:

1. **README.md** - User-facing features
2. **docs/QUICKSTART.md** - Usage examples
3. **docs/API_COMPATIBILITY.md** - If it affects API
4. **Code comments** - For complex logic
5. **This file** - If it's a pattern others should follow

### Documentation Style

- **Be concise** - Get to the point quickly
- **Use examples** - Show, don't just tell
- **Keep it updated** - Update docs when code changes
- **Cross-reference** - Link related documents

## Submitting Changes

### Before Submitting

- ✅ Code compiles without errors: `cargo build --release`
- ✅ No clippy warnings: `cargo clippy`
- ✅ Code is formatted: `cargo fmt`
- ✅ Tests pass: `cargo test`
- ✅ Manual testing done
- ✅ Documentation updated
- ✅ Commit message is clear

### Pull Request Checklist

- [ ] Descriptive title
- [ ] Explanation of what and why
- [ ] Screenshots/examples if UI changes
- [ ] Tests added/updated
- [ ] Documentation updated
- [ ] No breaking changes (or clearly documented)
- [ ] Backwards compatible with Python API (if applicable)

## Future Extensibility Ideas

Some ideas for future contributions:

### New Page Types

- **Multi-select list** - Choose multiple items
- **Radio group** - Select one from many
- **Checkbox list** - Multiple checkboxes
- **Date picker** - Select a date
- **Time picker** - Select a time
- **Color picker** - Choose a color
- **Slider** - Select from a range

### New Validation Presets

- **mac_address** - MAC address validation
- **uuid** - UUID validation
- **credit_card** - Credit card number (Luhn algorithm)
- **date** - Date format validation
- **time** - Time format validation
- **zip_code** - Zip/postal code validation
- **ssn** - Social security number (with masking)

### Other Improvements

- **Themes** - Dark mode, custom colors
- **Icons** - Custom icons for page types
- **Animations** - Smooth transitions
- **Keyboard shortcuts** - Better navigation
- **Native file dialogs** - Using `rfd` crate
- **Progress indicators** - Animated progress
- **Form validation** - Real-time validation feedback

## Questions?

If you have questions about contributing:

1. Check the documentation in `docs/`
2. Look at existing code for examples
3. Open an issue to discuss your idea
4. Start small - even fixing typos helps!

## License

By contributing, you agree that your contributions will be licensed under the Apache 2.0 License.

## Thank You!

Every contribution, no matter how small, makes Simple Wizard better. Thank you for your interest in improving the project! 🎉
