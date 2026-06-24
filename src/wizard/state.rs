//! Wizard state management and core logic
//!
//! This module contains the main implementation of WizardWindow, including:
//! - Window initialization and state management
//! - Message handling and validation
//! - Socket command processing
//! - Response building and sending

use iced::{Subscription, Theme};
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::{Mutex, oneshot};

use crate::pages::ValidationPresets;
use super::types::{Message, PageType, WizardWindow, CurrentPage};

impl WizardWindow {
    pub fn new(response_sender: Arc<Mutex<Option<oneshot::Sender<Value>>>>) -> Self {
        Self {
            info_title: "Installation Wizard".to_string(),
            info_description: "Follow the steps to complete the installation.".to_string(),
            info_help: String::new(),
            total_steps: 0,
            current_step: 0,
            status_text: "Ready".to_string(),
            log_messages: Vec::new(),
            current_page: None,
            text_input: String::new(),
            password_input: String::new(),
            confirm_password_input: String::new(),
            file_path: String::new(),
            validation_error: None,
            response_sender,
        }
    }

    pub fn title(&self) -> String {
        "Installation Wizard".to_string()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::UpdateInfo(title, description, help) => {
                if !title.is_empty() {
                    self.info_title = title;
                }
                if !description.is_empty() {
                    self.info_description = description;
                }
                if !help.is_empty() {
                    self.info_help = help;
                }
            }
            Message::UpdateProgress(current, total, status) => {
                if current >= 0 {
                    self.current_step = current;
                }
                if total >= 0 {
                    self.total_steps = total;
                }
                if !status.is_empty() {
                    self.status_text = status;
                }
            }
            Message::AppendLog(msg) => {
                self.log_messages.push(msg);
            }
            Message::ClearLog => {
                self.log_messages.clear();
            }
            Message::ShowPage(page_type, params) => {
                self.show_page(page_type, params);
            }
            Message::TextChanged(value) => {
                self.text_input = value;
                self.validation_error = None;
            }
            Message::PasswordChanged(value) => {
                self.password_input = value;
                self.validation_error = None;
            }
            Message::ConfirmPasswordChanged(value) => {
                self.confirm_password_input = value;
                self.validation_error = None;
            }
            Message::FilePathChanged(value) => {
                self.file_path = value;
            }
            Message::BrowseFile => {
                // Open native file picker
                if let Some(file) = rfd::FileDialog::new().pick_file() {
                    self.file_path = file.to_string_lossy().to_string();
                }
            }
            Message::BrowseDirectory => {
                // Open native directory picker
                if let Some(dir) = rfd::FileDialog::new().pick_folder() {
                    self.file_path = dir.to_string_lossy().to_string();
                }
            }
            Message::NextClicked => {
                if let Some(page) = &self.current_page {
                    match page.page_type {
                        PageType::Password if page.confirm => {
                            if self.password_input != self.confirm_password_input {
                                self.validation_error = Some("Passwords do not match!".to_string());
                                return;
                            }
                        }
                        PageType::Text => {
                            if let Some(validate) = &page.validate {
                                if let Err(err) = self.validate_text(&self.text_input, validate, &page.validation_message) {
                                    self.validation_error = Some(err);
                                    return;
                                }
                            }
                        }
                        _ => {}
                    }

                    // Build response based on page type
                    let response = self.build_response("next");
                    self.send_response(response);
                }
            }
            Message::CancelClicked => {
                let response = self.build_response("cancel");
                self.send_response(response);
            }
            Message::ButtonClicked(button) => {
                let response = serde_json::json!({
                    "action": "button",
                    "button": button
                });
                self.send_response(response);
            }
            Message::FinishClicked => {
                let response = self.build_response("finish");
                self.send_response(response);
            }
            Message::SocketCommand(cmd) => {
                self.process_socket_command(cmd);
            }
            Message::Quit => {
                std::process::exit(0);
            }
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        // Socket server is handled by background thread, not subscription
        Subscription::none()
    }

    pub fn theme(&self) -> Theme {
        Theme::default()
    }

    pub(crate) fn process_socket_command(&mut self, cmd: Value) {
        let command = cmd.get("command").and_then(|c| c.as_str()).unwrap_or("");

        match command {
            "set_info" => {
                let title = cmd.get("title").and_then(|t| t.as_str()).unwrap_or("").to_string();
                let description = cmd.get("description").and_then(|d| d.as_str()).unwrap_or("").to_string();
                let help_text = cmd.get("help_text").and_then(|h| h.as_str()).unwrap_or("").to_string();

                if !title.is_empty() {
                    self.info_title = title;
                }
                if !description.is_empty() {
                    self.info_description = description;
                }
                if !help_text.is_empty() {
                    self.info_help = help_text;
                }
            }
            "set_progress" => {
                if let Some(current) = cmd.get("current").and_then(|c| c.as_i64()) {
                    self.current_step = current as i32;
                }
                if let Some(total) = cmd.get("total").and_then(|t| t.as_i64()) {
                    self.total_steps = total as i32;
                }
                if let Some(status) = cmd.get("status").and_then(|s| s.as_str()) {
                    self.status_text = status.to_string();
                }
            }
            "show_page" => {
                if let Some(page_type_str) = cmd.get("page_type").and_then(|p| p.as_str()) {
                    let page_type = match page_type_str {
                        "welcome" => PageType::Welcome,
                        "file" => PageType::File,
                        "directory" => PageType::Directory,
                        "password" => PageType::Password,
                        "question" => PageType::Question,
                        "text" => PageType::Text,
                        "warning" => PageType::Warning,
                        "error" => PageType::Error,
                        "complete" => PageType::Complete,
                        _ => return,
                    };

                    let params = cmd.get("params").cloned().unwrap_or(Value::Object(serde_json::Map::new()));
                    self.show_page(page_type, params);
                }
            }
            "append_log" => {
                if let Some(message) = cmd.get("message").and_then(|m| m.as_str()) {
                    self.log_messages.push(message.to_string());
                }
            }
            "clear_log" => {
                self.log_messages.clear();
            }
            "quit" => {
                std::process::exit(0);
            }
            _ => {
                eprintln!("Unknown command: {}", command);
            }
        }
    }

    pub(crate) fn show_page(&mut self, page_type: PageType, params: Value) {
        let title = params["title"].as_str().unwrap_or("").to_string();
        let message = params["message"].as_str().unwrap_or("").to_string();
        let buttons = params["buttons"].as_array()
            .map(|arr| arr.iter().filter_map(|v| v.as_str()).map(|s| s.to_string()).collect())
            .unwrap_or_else(|| vec!["Yes".to_string(), "No".to_string()]);
        let default_path = params["default_path"].as_str().unwrap_or("").to_string();
        let placeholder = params["placeholder"].as_str().unwrap_or("").to_string();
        let validate = params["validate"].as_str().map(|s| s.to_string());
        let validation_message = params["validation_message"].as_str().map(|s| s.to_string());
        let confirm = params["confirm"].as_bool().unwrap_or(true);

        self.current_page = Some(CurrentPage {
            page_type,
            title,
            message,
            buttons,
            default_path: default_path.clone(),
            placeholder,
            validate,
            validation_message,
            confirm,
        });

        // Reset input state
        self.text_input.clear();
        self.password_input.clear();
        self.confirm_password_input.clear();
        self.file_path = default_path;
        self.validation_error = None;
    }

    pub(crate) fn validate_text(&self, text: &str, validate: &str, custom_message: &Option<String>) -> Result<(), String> {
        if let Some((pattern, default_msg)) = ValidationPresets::get(validate) {
            let regex = regex::Regex::new(pattern).unwrap();
            if !regex.is_match(text) {
                return Err(custom_message.clone().unwrap_or_else(|| default_msg.to_string()));
            }
        } else {
            // Custom regex
            if let Ok(regex) = regex::Regex::new(validate) {
                if !regex.is_match(text) {
                    return Err(custom_message.clone().unwrap_or_else(|| "Please enter a valid value".to_string()));
                }
            }
        }
        Ok(())
    }

    pub(crate) fn build_response(&self, action: &str) -> Value {
        let mut response = serde_json::json!({
            "action": action
        });

        if let Some(page) = &self.current_page {
            match page.page_type {
                PageType::Password => {
                    response["password"] = serde_json::json!(self.password_input);
                }
                PageType::Text => {
                    response["text"] = serde_json::json!(self.text_input);
                }
                PageType::File | PageType::Directory => {
                    response["path"] = serde_json::json!(self.file_path);
                }
                _ => {}
            }
        }

        response
    }

    pub(crate) fn send_response(&self, response: Value) {
        let sender = self.response_sender.clone();
        // Spawn a task to send the response
        std::thread::spawn(move || {
            let rt = tokio::runtime::Handle::try_current()
                .unwrap_or_else(|_| tokio::runtime::Runtime::new().unwrap().handle().clone());
            rt.block_on(async {
                let mut guard = sender.lock().await;
                if let Some(tx) = guard.take() {
                    let _ = tx.send(response);
                }
            });
        });
    }
}
