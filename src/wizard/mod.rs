//! Wizard GUI module
//!
//! This module implements the wizard GUI using the iced framework.
//! It provides a multi-page wizard interface with:
//! - Information and progress panels
//! - Various page types (welcome, file, directory, password, question, text, warning, error, complete)
//! - Unix socket server for external control
//! - Input validation
//!
//! The module is organized into:
//! - types: Core types (Message, PageType, WizardWindow, CurrentPage)
//! - state: State management and message handling
//! - ui: User interface components (panels and pages)
//! - socket: Socket server for external communication
//! - app: Main application entry point

pub mod types;
pub mod state;
pub mod ui;
pub mod socket;
pub mod app;

// Re-export the main entry point
pub use app::run_wizard;

// Re-export core types for external use
pub use types::{Message, PageType, WizardWindow};
