//! Core types for the wizard
//!
//! This module contains the fundamental types used throughout the wizard:
//! - Message: Events that drive the wizard's state machine
//! - PageType: The different kinds of pages that can be displayed
//! - WizardWindow: The main application state
//! - CurrentPage: The currently displayed page configuration

use serde_json::Value;
use std::sync::Arc;
use tokio::sync::{Mutex, oneshot};

/// Messages that drive the wizard's event loop
///
/// These messages represent all possible events in the wizard, from user
/// interactions (clicks, text input) to socket commands and system events.
#[derive(Debug, Clone)]
pub enum Message {
    // Page navigation
    NextClicked,
    CancelClicked,
    ButtonClicked(String),
    FinishClicked,

    // Input changes
    TextChanged(String),
    PasswordChanged(String),
    ConfirmPasswordChanged(String),
    FilePathChanged(String),

    // File selection
    BrowseFile,
    BrowseDirectory,
    FileSelected(Option<std::path::PathBuf>),
    DirectorySelected(Option<std::path::PathBuf>),

    // Progress updates
    UpdateProgress(i32, i32, String),
    UpdateInfo(String, String, String),
    AppendLog(String),
    ClearLog,
    ToggleLog,

    // Page transitions
    ShowPage(PageType, serde_json::Value),

    // Socket server events
    SocketCommand(Value),

    Quit,
}

/// Types of pages that can be displayed in the wizard
#[derive(Debug, Clone)]
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

/// Main wizard window state
///
/// This struct holds all the state for the wizard application, including:
/// - Information panel content (title, description, help)
/// - Progress tracking (current step, total steps, status)
/// - Log messages
/// - Current page configuration
/// - User input state
/// - Response callback for socket communication
pub struct WizardWindow {
    // Info panel
    pub(crate) info_title: String,
    pub(crate) info_description: String,
    pub(crate) info_help: String,

    // Progress
    pub(crate) total_steps: i32,
    pub(crate) current_step: i32,
    pub(crate) status_text: String,

    // Log
    pub(crate) log_messages: Vec<String>,
    pub(crate) log_scroll_id: iced::widget::scrollable::Id,
    pub(crate) log_expanded: bool,

    // Current page
    pub(crate) current_page: Option<CurrentPage>,

    // Input state
    pub(crate) text_input: String,
    pub(crate) password_input: String,
    pub(crate) confirm_password_input: String,
    pub(crate) file_path: String,

    // Validation
    pub(crate) validation_error: Option<String>,

    // Response callback for socket communication
    pub(crate) response_sender: Arc<Mutex<Option<oneshot::Sender<Value>>>>,
}

/// Configuration for the currently displayed page
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub(crate) struct CurrentPage {
    pub(crate) page_type: PageType,
    pub(crate) title: String,
    pub(crate) message: String,
    pub(crate) buttons: Vec<String>,
    pub(crate) default_path: String,
    pub(crate) placeholder: String,
    pub(crate) validate: Option<String>,
    pub(crate) validation_message: Option<String>,
    pub(crate) confirm: bool,
    pub(crate) min_length: usize,
}
