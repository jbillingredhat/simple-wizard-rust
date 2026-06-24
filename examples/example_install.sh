#!/bin/bash
#
# Example installation script using Simple Wizard
#
# This demonstrates how to use the wizard from a bash script to create
# an interactive installer for your application.
#
# Usage:
#   1. Start the wizard: ./target/release/simple-wizard
#   2. Run this script: ./examples/example_install.sh

set -e  # Exit on error

# Configuration
SOCKET="/tmp/simple-wizard.sock"
CLIENT="./target/release/simple-wizard-client"

# Check if wizard is running
if [ ! -S "$SOCKET" ]; then
    echo "Error: Wizard is not running!"
    echo "Please start the wizard first: ./target/release/simple-wizard"
    exit 1
fi

# Helper function to call the client
call_wizard() {
    $CLIENT "$@"
}

# Helper function to get response value from JSON
# Requires jq to be installed
get_response() {
    if command -v jq &> /dev/null; then
        echo "$1" | jq -r ".response.$2 // empty"
    else
        # Fallback: simple grep parsing (less robust)
        echo "$1" | grep -oP "\"$2\":\s*\"\K[^\"]*" || echo ""
    fi
}

echo "=========================================="
echo "  Example Application Installer"
echo "=========================================="
echo ""

# Set up the wizard information
call_wizard set-info \
    --title "Example Application Installer" \
    --description "This wizard will guide you through installing Example App." \
    --help-text "Follow the prompts to complete the installation. You can cancel at any time."

# Set total number of steps for progress tracking
call_wizard set-progress --total 7 --current 0 --status "Starting installation"

echo "Step 1: Welcome"
# Step 1: Welcome page
call_wizard set-progress --current 1 --status "Welcome"
response=$(call_wizard welcome \
    --title "Welcome to Example App Installer" \
    --message "This wizard will help you install Example App on your system.

Example App is a demonstration application that shows how to use
Simple Wizard for creating interactive installers.

Click Next to begin the installation process.")

echo "  ✓ User acknowledged welcome"

echo ""
echo "Step 2: Installation Directory"
# Step 2: Ask for installation directory
call_wizard set-progress --current 2 --status "Selecting installation directory"
response=$(call_wizard directory \
    --title "Installation Directory" \
    --message "Select where you want to install Example App." \
    --default "$HOME/example-app")

INSTALL_DIR=$(get_response "$response" "path")
echo "  ✓ Installation directory: $INSTALL_DIR"

# Check if user cancelled
if [ -z "$INSTALL_DIR" ]; then
    call_wizard error \
        --title "Installation Cancelled" \
        --message "No directory selected. Installation cannot continue."
    call_wizard quit
    exit 1
fi

echo ""
echo "Step 3: User Information"
# Step 3: Ask for user's email (with validation)
call_wizard set-progress --current 3 --status "Getting user information"
response=$(call_wizard text \
    --title "User Information" \
    --message "Please enter your email address for registration and updates." \
    --placeholder "user@example.com" \
    --validate "email")

EMAIL=$(get_response "$response" "text")
echo "  ✓ Email: $EMAIL"

echo ""
echo "Step 4: Configuration Password"
# Step 4: Set admin password
call_wizard set-progress --current 4 --status "Security setup"
response=$(call_wizard password \
    --title "Admin Password" \
    --message "Create an administrator password for Example App." \
    --confirm)

echo "  ✓ Admin password set"

echo ""
echo "Step 5: Installation Type"
# Step 5: Ask a question with custom buttons
call_wizard set-progress --current 5 --status "Configuration options"
response=$(call_wizard question \
    --title "Installation Type" \
    --message "Which type of installation would you prefer?" \
    --buttons "Full" "Minimal" "Custom")

INSTALL_TYPE=$(get_response "$response" "button")
echo "  ✓ Installation type: $INSTALL_TYPE"

echo ""
echo "Step 6: Desktop Integration"
# Step 6: Desktop shortcut question
response=$(call_wizard question \
    --title "Desktop Shortcut" \
    --message "Would you like to create a desktop shortcut?" \
    --buttons "Yes" "No" "Ask me later")

SHORTCUT=$(get_response "$response" "button")
echo "  ✓ Desktop shortcut: $SHORTCUT"

echo ""
echo "Step 7: Installing Files"
# Step 7: Show warning before installation
call_wizard set-progress --current 6 --status "Installing files"
call_wizard warning \
    --title "Ready to Install" \
    --message "The installation will now copy files to your system.

Installation directory: $INSTALL_DIR
Installation type: $INSTALL_TYPE
Desktop shortcut: $SHORTCUT

This may take a few moments. Click OK to continue."

# Simulate installation with progress logging
call_wizard log --message "=========================================="
call_wizard log --message "Starting installation process"
call_wizard log --message "=========================================="
call_wizard log --message ""
call_wizard log --message "Installation directory: $INSTALL_DIR"
call_wizard log --message "Installation type: $INSTALL_TYPE"
call_wizard log --message "Email: $EMAIL"
call_wizard log --message ""

call_wizard log --message "[1/5] Creating directory structure..."
sleep 1

call_wizard log --message "[2/5] Copying application files..."
sleep 1.5

call_wizard log --message "[3/5] Setting up configuration..."
call_wizard log --message "      - Registration email: $EMAIL"
call_wizard log --message "      - Desktop shortcut: $SHORTCUT"
sleep 1

call_wizard log --message "[4/5] Configuring permissions..."
sleep 1

call_wizard log --message "[5/5] Finalizing installation..."
sleep 1

call_wizard log --message ""
call_wizard log --message "Installation completed successfully!"
call_wizard log --message "=========================================="

# Step 8: Completion
call_wizard set-progress --current 7 --status "Complete"
call_wizard complete \
    --title "Installation Complete!" \
    --message "Example App has been successfully installed!

Installation details:
  Location: $INSTALL_DIR
  Type: $INSTALL_TYPE
  Email: $EMAIL
  Desktop shortcut: $SHORTCUT

You can now launch Example App from your applications menu."

echo ""
echo "=========================================="
echo "  Installation Completed Successfully!"
echo "=========================================="

# Clean up and quit
call_wizard quit

echo ""
echo "Thank you for installing Example App!"
