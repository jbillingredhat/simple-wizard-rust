use simple_wizard::WizardClient;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = WizardClient::new("/tmp/simple-wizard.sock");

    // Set up the wizard information
    client.set_info(
        Some("Rust App Installer"),
        Some("This wizard will guide you through installing Rust App."),
        Some("Follow the prompts to complete the installation."),
    )?;

    // Set total number of steps
    client.set_progress(Some(0), Some(7), Some("Starting installation"))?;

    // Step 1: Welcome
    client.set_progress(Some(1), None, Some("Welcome"))?;
    let response = client.show_welcome(
        "Welcome to Rust App Installer",
        "This wizard will help you install Rust App on your system.\n\n\
         Click Next to begin the installation process.",
    )?;
    println!("Welcome response: {:?}", response);

    // Step 2: Select installation directory
    client.set_progress(Some(2), None, Some("Selecting installation directory"))?;
    let response = client.show_directory(
        "Installation Directory",
        "Select where you want to install Rust App.",
        &format!("{}/rust-app", std::env::var("HOME").unwrap_or_default()),
    )?;

    if response.data.get("response")
        .and_then(|r| r.get("action"))
        .and_then(|a| a.as_str())
        == Some("cancel")
    {
        client.show_error("Installation Cancelled", "Installation was cancelled by user.")?;
        client.quit()?;
        return Ok(());
    }

    let install_dir = response.data.get("response")
        .and_then(|r| r.get("path"))
        .and_then(|p| p.as_str())
        .unwrap_or("");
    println!("Installation directory: {}", install_dir);

    // Step 3: Select configuration file (optional)
    client.set_progress(Some(3), None, Some("Configuration file"))?;
    let response = client.show_file(
        "Configuration File",
        "Select a configuration file (optional).",
        "",
    )?;
    let config_file = response.data.get("response")
        .and_then(|r| r.get("path"))
        .and_then(|p| p.as_str())
        .unwrap_or("");
    println!("Configuration file: {}", config_file);

    // Step 4: Password setup
    client.set_progress(Some(4), None, Some("Setting up password"))?;
    let response = client.show_password(
        "Set Admin Password",
        "Create a password for the admin account.",
        true,
    )?;

    if response.data.get("response")
        .and_then(|r| r.get("action"))
        .and_then(|a| a.as_str())
        == Some("cancel")
    {
        client.show_error("Installation Cancelled", "Installation was cancelled by user.")?;
        client.quit()?;
        return Ok(());
    }

    println!("Password set successfully");

    // Step 5: User information (with email validation)
    client.set_progress(Some(5), None, Some("User information"))?;
    let response = client.show_text(
        "User Information",
        "Please enter your email address.",
        "",
        "user@example.com",
        Some("email"),
        None,
    )?;
    let email = response.data.get("response")
        .and_then(|r| r.get("text"))
        .and_then(|t| t.as_str())
        .unwrap_or("");
    println!("Email: {}", email);

    // Step 6: Installation options
    client.set_progress(Some(6), None, Some("Installation options"))?;
    let response = client.show_question(
        "Installation Type",
        "What type of installation would you like?",
        vec!["Full".to_string(), "Minimal".to_string(), "Custom".to_string()],
    )?;
    let install_type = response.data.get("response")
        .and_then(|r| r.get("button"))
        .and_then(|b| b.as_str())
        .unwrap_or("");
    println!("Installation type: {}", install_type);

    // Show warning before proceeding
    client.show_warning(
        "Ready to Install",
        &format!(
            "The installer is ready to install Rust App.\n\n\
             Installation directory: {}\n\
             Installation type: {}\n\n\
             Click OK to continue.",
            install_dir, install_type
        ),
    )?;

    // Simulate installation with logging
    client.append_log(&format!("Starting installation to {}", install_dir))?;
    client.append_log("Creating directory structure...")?;
    thread::sleep(Duration::from_secs(1));
    client.append_log("Installing Rust packages...")?;
    thread::sleep(Duration::from_secs(1));
    client.append_log(&format!("Configuring for {} installation...", install_type))?;
    client.append_log(&format!("Setting user email to {}", email))?;
    thread::sleep(Duration::from_secs(1));
    client.append_log("Installation process complete!")?;

    // Step 7: Complete
    client.set_progress(Some(7), None, Some("Installation complete"))?;
    client.show_complete(
        "Installation Complete!",
        &format!(
            "Rust App has been successfully installed.\n\n\
             Installation directory: {}\n\
             Email: {}\n\
             Installation type: {}\n\n\
             Thank you for installing Rust App!",
            install_dir, email, install_type
        ),
    )?;

    // Quit the wizard
    client.quit()?;

    Ok(())
}
