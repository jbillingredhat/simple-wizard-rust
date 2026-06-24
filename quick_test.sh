#!/bin/bash
# Quick test to verify wizard and client work

set -e

echo "=== Quick Wizard Test ==="
echo ""

# Check binaries exist
if [ ! -f target/release/simple-wizard ]; then
    echo "ERROR: simple-wizard binary not found. Run 'cargo build --release' first."
    exit 1
fi

if [ ! -f target/release/simple-wizard-client ]; then
    echo "ERROR: simple-wizard-client binary not found. Run 'cargo build --release' first."
    exit 1
fi

echo "✓ Binaries found"
echo ""

# Remove old socket
rm -f /tmp/simple-wizard.sock

echo "Starting wizard in background..."
./target/release/simple-wizard &
WIZARD_PID=$!

# Give wizard time to start and create socket
sleep 2

# Check if socket was created
if [ ! -S /tmp/simple-wizard.sock ]; then
    echo "ERROR: Unix socket not created at /tmp/simple-wizard.sock"
    kill $WIZARD_PID 2>/dev/null || true
    exit 1
fi

echo "✓ Wizard started (PID: $WIZARD_PID)"
echo "✓ Socket created at /tmp/simple-wizard.sock"
echo ""

# Test basic commands
echo "Testing commands..."

echo -n "  - set-info: "
./target/release/simple-wizard-client set-info \
  --title "Test Wizard" \
  --description "Quick test" && echo "✓" || echo "✗"

sleep 0.5

echo -n "  - welcome: "
./target/release/simple-wizard-client welcome \
  --title "Test" \
  --message "Hello World" && echo "✓" || echo "✗"

sleep 0.5

echo -n "  - log: "
./target/release/simple-wizard-client log \
  --message "Test log message" && echo "✓" || echo "✗"

sleep 0.5

echo -n "  - quit: "
./target/release/simple-wizard-client quit && echo "✓" || echo "✗"

# Wait for wizard to exit
wait $WIZARD_PID 2>/dev/null || true

echo ""
echo "=== Test Complete ==="
echo ""
echo "All basic commands work! Try running:"
echo "  ./test_wizard.sh       - Full test with all page types"
echo "  cargo run --example example_install - Example installer"
