# API Compatibility Analysis

## Summary

✅ **The Rust implementation is 100% compatible with the Python API**

Both implementations use the exact same JSON protocol over Unix domain sockets at `/tmp/simple-wizard.sock`.

## Command Protocol Comparison

### Command Format

**Both versions use identical JSON format:**
```json
{
  "command": "command_name",
  "param1": "value1",
  "param2": "value2"
}
```

### Commands Supported

| Command | Python | Rust | Notes |
|---------|--------|------|-------|
| `set_info` | ✅ | ✅ | Identical |
| `set_progress` | ✅ | ✅ | Identical |
| `show_page` | ✅ | ✅ | Identical |
| `append_log` | ✅ | ✅ | Identical |
| `clear_log` | ✅ | ✅ | Identical |
| `quit` | ✅ | ✅ | Identical |

### Page Types

| Page Type | Python | Rust | Notes |
|-----------|--------|------|-------|
| `welcome` | ✅ | ✅ | Identical |
| `file` | ✅ | ✅ | Identical |
| `directory` | ✅ | ✅ | Identical |
| `password` | ✅ | ✅ | Identical |
| `question` | ✅ | ✅ | Identical |
| `text` | ✅ | ✅ | Identical |
| `warning` | ✅ | ✅ | Identical |
| `error` | ✅ | ✅ | Identical |
| `complete` | ✅ | ✅ | Identical |

## Detailed Command Comparison

### 1. set_info

**Python:**
```json
{
  "command": "set_info",
  "title": "My App",
  "description": "Description text",
  "help_text": "Help text"
}
```

**Rust:**
```json
{
  "command": "set_info",
  "title": "My App",
  "description": "Description text",
  "help_text": "Help text"
}
```

✅ **Identical**

### 2. set_progress

**Python:**
```json
{
  "command": "set_progress",
  "current": 3,
  "total": 10,
  "status": "Installing packages"
}
```

**Rust:**
```json
{
  "command": "set_progress",
  "current": 3,
  "total": 10,
  "status": "Installing packages"
}
```

✅ **Identical**

### 3. show_page (welcome example)

**Python:**
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

**Rust:**
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

✅ **Identical**

### 4. show_page (directory example)

**Python:**
```json
{
  "command": "show_page",
  "page_type": "directory",
  "params": {
    "title": "Select Directory",
    "message": "Choose install location",
    "default_path": "/home/user/app"
  }
}
```

**Rust:**
```json
{
  "command": "show_page",
  "page_type": "directory",
  "params": {
    "title": "Select Directory",
    "message": "Choose install location",
    "default_path": "/home/user/app"
  }
}
```

✅ **Identical**

### 5. show_page (text with validation)

**Python:**
```json
{
  "command": "show_page",
  "page_type": "text",
  "params": {
    "title": "Enter Email",
    "message": "Please enter your email address",
    "placeholder": "user@example.com",
    "validate": "email",
    "validation_message": "Please enter a valid email"
  }
}
```

**Rust:**
```json
{
  "command": "show_page",
  "page_type": "text",
  "params": {
    "title": "Enter Email",
    "message": "Please enter your email address",
    "placeholder": "user@example.com",
    "validate": "email",
    "validation_message": "Please enter a valid email"
  }
}
```

✅ **Identical**

### 6. append_log

**Python:**
```json
{
  "command": "append_log",
  "message": "Installing packages..."
}
```

**Rust:**
```json
{
  "command": "append_log",
  "message": "Installing packages..."
}
```

✅ **Identical**

### 7. quit

**Python:**
```json
{
  "command": "quit"
}
```

**Rust:**
```json
{
  "command": "quit"
}
```

✅ **Identical**

## Response Format Comparison

### Success Response (non-interactive commands)

**Python:**
```json
{
  "status": "ok"
}
```

**Rust:**
```json
{
  "status": "ok"
}
```

✅ **Identical**

### Success Response (show_page commands)

**Python:**
```json
{
  "status": "ok",
  "response": {
    "action": "next",
    "text": "user@example.com"
  }
}
```

**Rust:**
```json
{
  "status": "ok",
  "response": {
    "action": "next",
    "text": "user@example.com"
  }
}
```

✅ **Identical**

### Error Response

**Python:**
```json
{
  "status": "error",
  "message": "Connection failed"
}
```

**Rust:**
```json
{
  "status": "error",
  "message": "Connection failed"
}
```

✅ **Identical**

## Client Library API Comparison

### Python Client

```python
from simple_wizard import WizardClient

client = WizardClient()

# Set info
client.set_info(title="My App", description="Description")

# Set progress
client.set_progress(current=1, total=5, status="Starting")

# Show pages
client.show_welcome("Welcome", "Welcome message")
client.show_file("Select File", "Choose a file", "/default/path")
client.show_directory("Select Dir", "Choose a directory", "/home")
client.show_password("Password", "Enter password", confirm=True)
client.show_question("Question?", "Choose one", buttons=["Yes", "No"])
client.show_text("Email", "Enter email", validate="email")
client.show_warning("Warning", "This is a warning")
client.show_error("Error", "An error occurred")
client.show_complete("Done", "Installation complete")

# Log
client.append_log("Log message")
client.clear_log()

# Quit
client.quit()
```

### Rust Client

```rust
use simple_wizard::WizardClient;

let client = WizardClient::new("/tmp/simple-wizard.sock");

// Set info
client.set_info(Some("My App"), Some("Description"), None)?;

// Set progress
client.set_progress(Some(1), Some(5), Some("Starting"))?;

// Show pages
client.show_welcome("Welcome", "Welcome message")?;
client.show_file("Select File", "Choose a file", "/default/path")?;
client.show_directory("Select Dir", "Choose a directory", "/home")?;
client.show_password("Password", "Enter password", true)?;
client.show_question("Question?", "Choose one", vec!["Yes", "No"])?;
client.show_text("Email", "Enter email", "", "", Some("email"), None)?;
client.show_warning("Warning", "This is a warning")?;
client.show_error("Error", "An error occurred")?;
client.show_complete("Done", "Installation complete")?;

// Log
client.append_log("Log message")?;
client.clear_log()?;

// Quit
client.quit()?;
```

### Differences

The API is **functionally identical**, with these language-specific differences:

1. **Error Handling:**
   - Python: Exceptions
   - Rust: `Result<T, E>` with `?` operator

2. **Optional Parameters:**
   - Python: `param=None`
   - Rust: `Some(value)` or `None`

3. **Lists:**
   - Python: `buttons=["Yes", "No"]`
   - Rust: `vec!["Yes", "No"]`

These are **natural language differences**, not API differences. The wire protocol is 100% identical.

## CLI Command Comparison

### Python CLI

```bash
simple-wizard-client set-info --title "My App" --description "Description"
simple-wizard-client set-progress --current 1 --total 5 --status "Starting"
simple-wizard-client welcome --title "Welcome" --message "Hello"
simple-wizard-client file --title "Select File" --default "/path"
simple-wizard-client directory --title "Select Dir" --default "/home"
simple-wizard-client password --title "Password" --confirm
simple-wizard-client question --title "Question?" --buttons Yes No
simple-wizard-client text --title "Email" --validate email
simple-wizard-client warning --title "Warning" --message "Warning text"
simple-wizard-client error --title "Error" --message "Error text"
simple-wizard-client complete --title "Done" --message "Complete"
simple-wizard-client log --message "Log message"
simple-wizard-client quit
```

### Rust CLI

```bash
simple-wizard-client set-info --title "My App" --description "Description"
simple-wizard-client set-progress --current 1 --total 5 --status "Starting"
simple-wizard-client welcome --title "Welcome" --message "Hello"
simple-wizard-client file --title "Select File" --default "/path"
simple-wizard-client directory --title "Select Dir" --default "/home"
simple-wizard-client password --title "Password" --confirm
simple-wizard-client question --title "Question?" --buttons Yes No
simple-wizard-client text --title "Email" --validate email
simple-wizard-client warning --title "Warning" --message "Warning text"
simple-wizard-client error --title "Error" --message "Error text"
simple-wizard-client complete --title "Done" --message "Complete"
simple-wizard-client log --message "Log message"
simple-wizard-client quit
```

✅ **100% Identical**

## Validation Presets

Both versions support the same validation presets:

| Preset | Python | Rust | Notes |
|--------|--------|------|-------|
| `email` | ✅ | ✅ | Email address validation |
| `url` | ✅ | ✅ | HTTP/HTTPS URLs |
| `ipv4` | ✅ | ✅ | IPv4 addresses |
| `port` | ✅ | ✅ | Port numbers (1-65535) |
| `hostname` | ✅ | ✅ | Valid hostnames |
| `username` | ✅ | ✅ | Usernames (3-32 chars) |
| `number` | ✅ | ✅ | Any integer |
| `positive_number` | ✅ | ✅ | Positive integers only |
| `alphanumeric` | ✅ | ✅ | Letters and numbers only |

✅ **All presets identical**

## Synchronization Model

Both versions use the same synchronization approach:

### Python
```python
# threading.Event for blocking
response_event = threading.Event()

def callback(resp):
    response_data['response'] = resp
    response_event.set()

# Wait for user (blocks socket handler)
response_event.wait(timeout=300)
```

### Rust
```rust
// oneshot channel for blocking
let (tx, rx) = oneshot::channel();

// Store sender
*response_sender.lock().await = Some(tx);

// Wait for user (blocks socket handler)
match tokio::time::timeout(Duration::from_secs(300), rx).await {
    Ok(Ok(response)) => { /* got response */ }
    ...
}
```

**Result:** Both block the socket connection until the user responds or timeout (300 seconds).

✅ **Functionally identical**

## Cross-Compatibility Test

You can use the **Python client with the Rust server** or vice versa:

### Python client → Rust server
```bash
# Start Rust server
./target/release/simple-wizard

# Use Python client
python -m simple_wizard.client welcome --title "Test" --message "Hello"
```

✅ **Works perfectly**

### Rust client → Python server
```bash
# Start Python server
python -m simple_wizard.wizard

# Use Rust client
./target/release/simple-wizard-client welcome --title "Test" --message "Hello"
```

✅ **Works perfectly**

## Architecture Comparison

| Component | Python | Rust | Compatible |
|-----------|--------|------|------------|
| **GUI Framework** | GTK4 | iced | N/A |
| **Socket Type** | Unix domain socket | Unix domain socket | ✅ |
| **Socket Path** | `/tmp/simple-wizard.sock` | `/tmp/simple-wizard.sock` | ✅ |
| **Protocol** | JSON over newline-delimited stream | JSON over newline-delimited stream | ✅ |
| **Threading** | threading.Thread | tokio async tasks | ✅ (compatible) |
| **Response Wait** | threading.Event | oneshot channel | ✅ (compatible) |
| **Timeout** | 300 seconds | 300 seconds | ✅ |

## Conclusion

✅ **100% API Compatible**

The Rust implementation is a **drop-in replacement** for the Python version:

1. **Same JSON protocol** - byte-for-byte identical on the wire
2. **Same socket path** - `/tmp/simple-wizard.sock`
3. **Same commands** - all 6 commands identical
4. **Same page types** - all 9 page types identical
5. **Same validation presets** - all 9 presets identical
6. **Same CLI** - commands are identical
7. **Same behavior** - blocking until user responds
8. **Same timeout** - 300 seconds
9. **Cross-compatible** - Python client works with Rust server and vice versa

**Scripts written for the Python version will work with the Rust version without any modifications.**
