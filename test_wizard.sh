#!/bin/bash

# Test script for the wizard
# Start the wizard in the background, then send it commands

echo "Starting wizard..."
./target/release/simple-wizard &
WIZARD_PID=$!

# Wait for wizard to start
sleep 2

echo "Testing wizard client commands..."

# Set wizard info
echo "1. Setting wizard info..."
./target/release/simple-wizard-client set-info \
  --title "Test Installer" \
  --description "Testing the Rust wizard" \
  --help "This is a test"

sleep 1

# Set progress
echo "2. Setting progress..."
./target/release/simple-wizard-client set-progress \
  --total 5 --current 1 --status "Starting"

sleep 1

# Show welcome page
echo "3. Showing welcome page..."
./target/release/simple-wizard-client welcome \
  --title "Welcome!" \
  --message "This is a test of the Rust wizard"

sleep 2

# Show text entry
echo "4. Showing text entry..."
./target/release/simple-wizard-client text \
  --title "Enter Email" \
  --message "Please enter your email" \
  --placeholder "user@example.com" \
  --validate email

sleep 2

# Add log messages
echo "5. Adding log messages..."
./target/release/simple-wizard-client log --message "Test message 1"
./target/release/simple-wizard-client log --message "Test message 2"
./target/release/simple-wizard-client log --message "Test message 3"

sleep 2

# Show completion
echo "6. Showing completion..."
./target/release/simple-wizard-client complete \
  --title "Done!" \
  --message "Test completed successfully!"

sleep 3

# Quit
echo "7. Quitting..."
./target/release/simple-wizard-client quit

# Clean up
wait $WIZARD_PID 2>/dev/null

echo "Test complete!"
