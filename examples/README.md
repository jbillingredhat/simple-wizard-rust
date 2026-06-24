# Examples

This directory contains examples demonstrating how to use Simple Wizard.

## Shell Script Example (Recommended)

**`example_install.sh`** - Complete installation wizard in bash

This is the **primary example** showing how to use the wizard from a shell script. This is the typical use case - creating interactive installers for your applications.

### Running the Shell Example

```bash
# Terminal 1: Start the wizard
./target/release/simple-wizard

# Terminal 2: Run the example
./examples/example_install.sh
```

### What It Demonstrates

- ✅ Setting wizard information (title, description, help text)
- ✅ Progress tracking (7 steps)
- ✅ **Welcome page** - Introduction to installer
- ✅ **Directory selection** - Choose installation location
- ✅ **Text input with validation** - Email address (validated)
- ✅ **Password entry** - Admin password with confirmation
- ✅ **Question with custom buttons** - Installation type (Full/Minimal/Custom)
- ✅ **Question with custom buttons** - Desktop shortcut preference
- ✅ **Warning page** - Pre-installation confirmation
- ✅ **Log messages** - Installation progress
- ✅ **Complete page** - Success message
- ✅ Response parsing and using user input

This example shows **all major features** of Simple Wizard in a real-world installation scenario.

## Rust Library Example (Advanced)

**`example_install_lib.rs`** - Using the Rust client library

This example shows how to use the Simple Wizard Rust library API programmatically. This is useful if you're writing your installer in Rust instead of bash.

### Running the Rust Example

```bash
# Terminal 1: Start the wizard
./target/release/simple-wizard

# Terminal 2: Run the Rust example
cargo run --example example_install_lib
```

### When to Use This

Use the Rust library example if you're:
- Writing your installer in Rust
- Need type safety and compile-time checks
- Want to use Rust's error handling
- Building a complex installer with lots of logic

### When to Use the Shell Example

Use the shell script example if you're:
- Writing a typical installer script
- Want maximum portability
- Prefer bash scripting
- Need a simple, straightforward approach

**For most users, the shell script example is the recommended approach.**

## Comparison

| Feature | Shell Script | Rust Library |
|---------|--------------|--------------|
| **Language** | Bash | Rust |
| **Complexity** | Simple | More complex |
| **Type Safety** | No | Yes |
| **Portability** | High | Requires Rust |
| **Use Case** | Most installers | Complex installers |
| **Learning Curve** | Low | Medium |
| **Recommended** | ✅ Yes | For Rust developers |

## Creating Your Own Installer

### Using the Shell Script Approach

1. Copy `example_install.sh` as a template
2. Modify the steps to match your installation needs
3. Update the titles, messages, and validation
4. Add your actual installation commands
5. Run it!

### Using the Rust Library Approach

1. Add `simple_wizard` as a dependency to your `Cargo.toml`:
   ```toml
   [dependencies]
   simple_wizard = { path = "../path/to/simple-wizard-rust" }
   ```

2. Use the `WizardClient` API:
   ```rust
   use simple_wizard::WizardClient;
   
   let client = WizardClient::new("/tmp/simple-wizard.sock");
   client.show_welcome("Welcome", "Welcome to installer!")?;
   // ... more commands
   ```

3. See `example_install_lib.rs` for a complete example

## Tips

### Error Handling

**Shell:**
```bash
set -e  # Exit on error
# Check if wizard is running
if [ ! -S "$SOCKET" ]; then
    echo "Error: Wizard is not running!"
    exit 1
fi
```

**Rust:**
```rust
if let Err(e) = client.show_welcome("Title", "Message") {
    eprintln!("Error: {}", e);
    std::process::exit(1);
}
```

### Response Parsing

**Shell (with jq):**
```bash
response=$(call_wizard directory --title "Dir" --default "/home")
INSTALL_DIR=$(echo "$response" | jq -r '.response.path')
```

**Rust:**
```rust
let response = client.show_directory("Dir", "Message", "/home")?;
let install_dir = response.data.get("response")
    .and_then(|r| r.get("path"))
    .and_then(|p| p.as_str())
    .unwrap_or("");
```

### Progress Tracking

**Both:**
```bash
# Shell
call_wizard set-progress --total 5 --current 1 --status "Starting"

# Rust
client.set_progress(Some(1), Some(5), Some("Starting"))?;
```

## Testing Your Installer

1. Start the wizard: `./target/release/simple-wizard`
2. Run your script in another terminal
3. Walk through all the steps
4. Test edge cases:
   - Cancel at different steps
   - Invalid input (if using validation)
   - Different button choices
   - Empty inputs

## More Information

- See `../README.md` for full wizard documentation
- See `../docs/QUICKSTART.md` for quick start guide
- See `../docs/API_COMPATIBILITY.md` for API reference
- See `../CONTRIBUTING.md` for extending the wizard

## Questions?

- Check the documentation in `../docs/`
- Look at the test scripts: `../test_wizard.sh`, `../quick_test.sh`
- Read the Python example for more ideas: `/home/jbilling/code/simple-wizard/examples/`
