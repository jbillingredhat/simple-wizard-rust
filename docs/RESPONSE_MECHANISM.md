# Response Mechanism Implementation

## Problem

The original implementation was sending an immediate "ok" response to socket commands without waiting for the user to actually interact with the GUI. This meant that scripts couldn't receive the user's actual choice.

## Solution

Implemented a response waiting mechanism using Tokio oneshot channels, matching the Python version's threading.Event approach:

### Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Socket Client                            │
│                  (simple-wizard-client)                     │
└─────────────────────────┬───────────────────────────────────┘
                          │
                          │ 1. Send show_page command
                          ▼
┌─────────────────────────────────────────────────────────────┐
│              Socket Handler (tokio task)                    │
│                                                             │
│  • Creates oneshot channel                                 │
│  • Stores sender in shared state                           │
│  • Sends command to GUI                                    │
│  • WAITS on oneshot receiver (blocks connection)           │
└─────────────────────────┬───────────────────────────────────┘
                          │
                          │ 2. Forward to GUI
                          ▼
┌─────────────────────────────────────────────────────────────┐
│                  GUI (iced event loop)                      │
│                                                             │
│  • Displays page to user                                   │
│  • User clicks button                                      │
│  • Builds response with user data                          │
│  • Sends response through oneshot channel                  │
└─────────────────────────┬───────────────────────────────────┘
                          │
                          │ 3. Response sent back
                          ▼
┌─────────────────────────────────────────────────────────────┐
│              Socket Handler (unblocks)                      │
│                                                             │
│  • Receives response from oneshot                          │
│  • Wraps in JSON                                           │
│  • Sends back to client                                    │
└─────────────────────────┬───────────────────────────────────┘
                          │
                          │ 4. Response received
                          ▼
┌─────────────────────────────────────────────────────────────┐
│                    Socket Client                            │
│              Gets user's actual choice!                     │
└─────────────────────────────────────────────────────────────┘
```

## Implementation Details

### 1. Shared Response Sender

Added to `WizardWindow`:
```rust
response_sender: Arc<Mutex<Option<oneshot::Sender<Value>>>>
```

This is shared between:
- The GUI (to send responses when user clicks)
- The socket handler (to set up the oneshot channel)

### 2. Socket Handler Logic

For `show_page` commands:
1. Create a new oneshot channel: `let (tx, rx) = oneshot::channel()`
2. Store `tx` in the shared state
3. Send the command to GUI
4. **WAIT** on `rx` with a 300-second timeout
5. Send the response back to the client

For other commands (set-info, set-progress, log, etc.):
- Send immediately without waiting
- Return "ok" response right away

### 3. GUI Button Handlers

When user clicks:
- `NextClicked` → Build response with action="next" + user data
- `CancelClicked` → Build response with action="cancel"
- `ButtonClicked(name)` → Build response with button name
- `FinishClicked` → Build response with action="finish"

Each handler calls `send_response()` which:
1. Takes the oneshot sender from shared state
2. Sends the response through it
3. Unblocks the socket handler

### 4. Response Data

The response includes user input based on page type:
- **Password page**: includes `password` field
- **Text page**: includes `text` field
- **File/Directory page**: includes `path` field
- **Question page**: includes `button` field
- All pages: include `action` field ("next", "cancel", "finish")

## Comparison with Python Version

### Python (threading.Event):
```python
response_event = threading.Event()

def callback(resp):
    response_data['response'] = resp
    response_event.set()

self.window.response_callback = callback
GLib.idle_add(show_page_wrapper)
response_event.wait(timeout=300)  # BLOCKS here
```

### Rust (oneshot channel):
```rust
let (tx, rx) = oneshot::channel();

*response_sender.lock().await = Some(tx);
msg_sender.send(Message::SocketCommand(cmd));

// BLOCKS here
match tokio::time::timeout(Duration::from_secs(300), rx).await {
    Ok(Ok(response)) => { /* send response */ }
    ...
}
```

Both achieve the same goal: block the socket handler until the user interacts with the GUI.

## Testing

Run the test script to verify the mechanism works:

```bash
./test_response.sh
```

This will:
1. Start the wizard
2. Send a welcome page command
3. **Wait** (connection stays open)
4. You click "Next" in the GUI
5. Client receives the response with your action
6. Test completes

## Response Examples

### Welcome Page Response:
```json
{
  "status": "ok",
  "response": {
    "action": "next"
  }
}
```

### Text Entry Response:
```json
{
  "status": "ok",
  "response": {
    "action": "next",
    "text": "user@example.com"
  }
}
```

### Password Response:
```json
{
  "status": "ok",
  "response": {
    "action": "next",
    "password": "secretpass"
  }
}
```

### Directory Selection Response:
```json
{
  "status": "ok",
  "response": {
    "action": "next",
    "path": "/home/user/install"
  }
}
```

### Question Response:
```json
{
  "status": "ok",
  "response": {
    "action": "button",
    "button": "Full"
  }
}
```

## Timeout Handling

If the user doesn't respond within 5 minutes (300 seconds):
```json
{
  "status": "error",
  "message": "Response timeout"
}
```

This prevents clients from hanging forever if the wizard is closed or frozen.

## Key Files Modified

- `src/wizard.rs`:
  - Added `response_sender` field to `WizardWindow`
  - Updated button handlers to send responses
  - Added `build_response()` and `send_response()` helper methods
  - Modified `handle_connection()` to wait for responses on show_page commands
  - Updated `run_wizard()` to set up the shared response sender

## Result

✅ The Rust implementation now matches the Python version's behavior:
- Client commands **block** until user responds
- User input is captured and returned
- Timeout protection prevents hanging forever
- Scripts can make decisions based on user choices
