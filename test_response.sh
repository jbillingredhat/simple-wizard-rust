#!/bin/bash
# Test script to verify that the wizard waits for user responses

set -e

echo "=== Testing Response Mechanism ==="
echo ""

# Remove old socket
rm -f /tmp/simple-wizard.sock

echo "Starting wizard in background..."
./target/release/simple-wizard &
WIZARD_PID=$!

# Give wizard time to start
sleep 2

echo "✓ Wizard started (PID: $WIZARD_PID)"
echo ""

# Test welcome page - this should BLOCK until user clicks Next
echo "Sending welcome command (will wait for user to click Next)..."
echo "Please click the 'Next' button in the wizard window..."

# Run the client and capture the response
RESPONSE=$(./target/release/simple-wizard-client welcome \
  --title "Test Welcome" \
  --message "Click Next to continue" 2>&1)

echo ""
echo "Response received:"
echo "$RESPONSE"
echo ""

# Parse the response to see if we got user action
if echo "$RESPONSE" | grep -q "ok"; then
    echo "✓ Got response from wizard!"
else
    echo "✗ No response or error"
fi

echo ""
echo "Cleaning up..."
./target/release/simple-wizard-client quit 2>/dev/null || true
wait $WIZARD_PID 2>/dev/null || true

echo ""
echo "=== Test Complete ==="
