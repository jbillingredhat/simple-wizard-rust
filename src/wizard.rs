use iced::{
    widget::{button, column, container, row, text, text_input, progress_bar, scrollable, Column, Row},
    Element, Length, Theme, alignment, Alignment, Subscription,
};
use serde_json::Value;
use tokio::net::{UnixListener, UnixStream};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use std::sync::Arc;
use tokio::sync::{Mutex, oneshot};

use crate::pages::*;

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

    // Progress updates
    UpdateProgress(i32, i32, String),
    UpdateInfo(String, String, String),
    AppendLog(String),
    ClearLog,

    // Page transitions
    ShowPage(PageType, serde_json::Value),

    // Socket server events
    SocketCommand(Value),

    Quit,
}

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

pub struct WizardWindow {
    // Info panel
    info_title: String,
    info_description: String,
    info_help: String,

    // Progress
    total_steps: i32,
    current_step: i32,
    status_text: String,

    // Log
    log_messages: Vec<String>,

    // Current page
    current_page: Option<CurrentPage>,

    // Input state
    text_input: String,
    password_input: String,
    confirm_password_input: String,
    file_path: String,

    // Validation
    validation_error: Option<String>,

    // Response callback for socket communication
    response_sender: Arc<Mutex<Option<oneshot::Sender<Value>>>>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct CurrentPage {
    page_type: PageType,
    title: String,
    message: String,
    buttons: Vec<String>,
    default_path: String,
    placeholder: String,
    validate: Option<String>,
    validation_message: Option<String>,
    confirm: bool,
}

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
            _ => {}
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        // Socket server is handled by background thread, not subscription
        Subscription::none()
    }

    pub fn view(&self) -> Element<'_, Message> {
        let info_panel = self.build_info_panel();
        let content_area = self.build_content_area();
        let progress_panel = self.build_progress_panel();

        let main_content = row![
            container(info_panel)
                .width(250)
                .padding(12),
            container(content_area)
                .width(Length::Fill)
                .padding(12),
        ];

        let layout = column![
            main_content,
            progress_panel,
        ]
        .spacing(0);

        container(layout)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    pub fn theme(&self) -> Theme {
        Theme::default()
    }

    fn process_socket_command(&mut self, cmd: Value) {
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

    fn build_info_panel(&self) -> Column<'_, Message> {
        column![
            text(&self.info_title).size(18),
            text(&self.info_description).size(14),
            text(&self.info_help).size(12),
        ]
        .spacing(12)
        .padding(12)
    }

    fn build_content_area(&self) -> Element<'_, Message> {
        if let Some(page) = &self.current_page {
            match page.page_type {
                PageType::Welcome => self.build_welcome_page(page),
                PageType::File => self.build_file_page(page),
                PageType::Directory => self.build_directory_page(page),
                PageType::Password => self.build_password_page(page),
                PageType::Question => self.build_question_page(page),
                PageType::Text => self.build_text_page(page),
                PageType::Warning => self.build_warning_page(page),
                PageType::Error => self.build_error_page(page),
                PageType::Complete => self.build_complete_page(page),
            }
        } else {
            container(text("Ready").size(24))
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(alignment::Horizontal::Center)
                .align_y(alignment::Vertical::Center)
                .into()
        }
    }

    fn build_progress_panel(&self) -> Column<'_, Message> {
        let progress = if self.total_steps > 0 {
            self.current_step as f32 / self.total_steps as f32
        } else {
            0.0
        };

        let mut col = column![
            progress_bar(0.0..=1.0, progress),
            text(&self.status_text).size(12),
        ]
        .spacing(6)
        .padding(12);

        if !self.log_messages.is_empty() {
            let log_text = self.log_messages.join("\n");
            col = col.push(
                scrollable(text(log_text).size(10))
                    .height(150)
            );
        }

        col
    }

    fn build_welcome_page<'a>(&'a self, page: &'a CurrentPage) -> Element<'a, Message> {
        column![
            text(&page.title).size(24),
            text(&page.message).size(14),
            button(text("Next")).on_press(Message::NextClicked),
        ]
        .spacing(12)
        .padding(24)
        .into()
    }

    fn build_file_page<'a>(&'a self, page: &'a CurrentPage) -> Element<'a, Message> {
        let mut col = column![
            text(&page.title).size(20),
        ];

        if !page.message.is_empty() {
            col = col.push(text(&page.message).size(14));
        }

        col = col.push(
            row![
                text_input(&page.placeholder, &self.file_path)
                    .on_input(Message::FilePathChanged),
                button(text("Browse...")).on_press(Message::BrowseFile),
            ]
            .spacing(6)
        );

        col = col.push(
            row![
                button(text("Cancel")).on_press(Message::CancelClicked),
                button(text("Next")).on_press(Message::NextClicked),
            ]
            .spacing(6)
        );

        col.spacing(12).padding(24).into()
    }

    fn build_directory_page<'a>(&'a self, page: &'a CurrentPage) -> Element<'a, Message> {
        let mut col = column![
            text(&page.title).size(20),
        ];

        if !page.message.is_empty() {
            col = col.push(text(&page.message).size(14));
        }

        col = col.push(
            row![
                text_input(&page.placeholder, &self.file_path)
                    .on_input(Message::FilePathChanged),
                button(text("Browse...")).on_press(Message::BrowseDirectory),
            ]
            .spacing(6)
        );

        col = col.push(
            row![
                button(text("Cancel")).on_press(Message::CancelClicked),
                button(text("Next")).on_press(Message::NextClicked),
            ]
            .spacing(6)
        );

        col.spacing(12).padding(24).into()
    }

    fn build_password_page<'a>(&'a self, page: &'a CurrentPage) -> Element<'a, Message> {
        let mut col = column![
            text(&page.title).size(20),
        ];

        if !page.message.is_empty() {
            col = col.push(text(&page.message).size(14));
        }

        col = col.push(text("Password:"));
        col = col.push(
            text_input("", &self.password_input)
                .on_input(Message::PasswordChanged)
                .secure(true)
        );

        if page.confirm {
            col = col.push(text("Confirm Password:"));
            col = col.push(
                text_input("", &self.confirm_password_input)
                    .on_input(Message::ConfirmPasswordChanged)
                    .secure(true)
            );
        }

        if let Some(err) = &self.validation_error {
            col = col.push(text(err).size(12));
        }

        col = col.push(
            row![
                button(text("Cancel")).on_press(Message::CancelClicked),
                button(text("Next")).on_press(Message::NextClicked),
            ]
            .spacing(6)
        );

        col.spacing(12).padding(24).into()
    }

    fn build_question_page<'a>(&'a self, page: &'a CurrentPage) -> Element<'a, Message> {
        let mut col = column![
            text(&page.title).size(20),
            text(&page.message).size(14),
        ];

        let mut button_row = Row::new().spacing(6);
        for btn_text in &page.buttons {
            button_row = button_row.push(
                button(text(btn_text)).on_press(Message::ButtonClicked(btn_text.clone()))
            );
        }

        col = col.push(button_row);

        col.spacing(12).padding(24).into()
    }

    fn build_text_page<'a>(&'a self, page: &'a CurrentPage) -> Element<'a, Message> {
        let mut col = column![
            text(&page.title).size(20),
        ];

        if !page.message.is_empty() {
            col = col.push(text(&page.message).size(14));
        }

        col = col.push(
            text_input(&page.placeholder, &self.text_input)
                .on_input(Message::TextChanged)
        );

        if let Some(err) = &self.validation_error {
            col = col.push(text(err).size(12));
        }

        col = col.push(
            row![
                button(text("Cancel")).on_press(Message::CancelClicked),
                button(text("Next")).on_press(Message::NextClicked),
            ]
            .spacing(6)
        );

        col.spacing(12).padding(24).into()
    }

    fn build_warning_page<'a>(&'a self, page: &'a CurrentPage) -> Element<'a, Message> {
        column![
            text("⚠️").size(48),
            text(&page.title).size(20),
            text(&page.message).size(14),
            button(text("OK")).on_press(Message::NextClicked),
        ]
        .spacing(12)
        .padding(24)
        .align_x(Alignment::Center)
        .into()
    }

    fn build_error_page<'a>(&'a self, page: &'a CurrentPage) -> Element<'a, Message> {
        column![
            text("❌").size(48),
            text(&page.title).size(20),
            text(&page.message).size(14),
            button(text("OK")).on_press(Message::NextClicked),
        ]
        .spacing(12)
        .padding(24)
        .align_x(Alignment::Center)
        .into()
    }

    fn build_complete_page<'a>(&'a self, page: &'a CurrentPage) -> Element<'a, Message> {
        column![
            text("✓").size(48),
            text(&page.title).size(20),
            text(&page.message).size(14),
            button(text("Finish")).on_press(Message::FinishClicked),
        ]
        .spacing(12)
        .padding(24)
        .align_x(Alignment::Center)
        .into()
    }

    fn show_page(&mut self, page_type: PageType, params: Value) {
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

    fn validate_text(&self, text: &str, validate: &str, custom_message: &Option<String>) -> Result<(), String> {
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

    fn build_response(&self, action: &str) -> Value {
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

    fn send_response(&self, response: Value) {
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

// Background socket server
async fn run_socket_server(
    msg_sender: Arc<Mutex<Option<tokio::sync::mpsc::UnboundedSender<Message>>>>,
    response_sender: Arc<Mutex<Option<oneshot::Sender<Value>>>>
) {
    let socket_path = "/tmp/simple-wizard.sock";

    // Remove existing socket if it exists
    let _ = std::fs::remove_file(socket_path);

    let listener = match UnixListener::bind(socket_path) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Failed to bind socket: {}", e);
            return;
        }
    };

    println!("Socket server listening on {}", socket_path);

    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                let msg_sender = msg_sender.clone();
                let response_sender = response_sender.clone();
                tokio::spawn(async move {
                    handle_connection(stream, msg_sender, response_sender).await;
                });
            }
            Err(e) => {
                eprintln!("Accept error: {}", e);
                break;
            }
        }
    }
}

async fn handle_connection(
    mut stream: UnixStream,
    msg_sender: Arc<Mutex<Option<tokio::sync::mpsc::UnboundedSender<Message>>>>,
    response_sender: Arc<Mutex<Option<oneshot::Sender<Value>>>>
) {
    let (reader, mut writer) = stream.split();
    let mut reader = BufReader::new(reader);
    let mut line = String::new();

    if reader.read_line(&mut line).await.is_ok() {
        if let Ok(cmd) = serde_json::from_str::<Value>(&line) {
            let command = cmd.get("command").and_then(|c| c.as_str()).unwrap_or("");

            // For show_page commands, we need to wait for user response
            if command == "show_page" {
                // Create a oneshot channel for the response
                let (tx, rx) = oneshot::channel();

                // Store the sender in the shared state
                {
                    let mut guard = response_sender.lock().await;
                    *guard = Some(tx);
                }

                // Send command to GUI
                {
                    let guard = msg_sender.lock().await;
                    if let Some(msg_tx) = guard.as_ref() {
                        let _ = msg_tx.send(Message::SocketCommand(cmd));
                    }
                }

                // Wait for user response (with timeout)
                match tokio::time::timeout(std::time::Duration::from_secs(300), rx).await {
                    Ok(Ok(user_response)) => {
                        let response = serde_json::json!({
                            "status": "ok",
                            "response": user_response
                        });
                        let response_str = serde_json::to_string(&response).unwrap() + "\n";
                        let _ = writer.write_all(response_str.as_bytes()).await;
                    }
                    Ok(Err(_)) => {
                        // Channel closed without response
                        let response = serde_json::json!({
                            "status": "error",
                            "message": "No response received"
                        });
                        let response_str = serde_json::to_string(&response).unwrap() + "\n";
                        let _ = writer.write_all(response_str.as_bytes()).await;
                    }
                    Err(_) => {
                        // Timeout
                        let response = serde_json::json!({
                            "status": "error",
                            "message": "Response timeout"
                        });
                        let response_str = serde_json::to_string(&response).unwrap() + "\n";
                        let _ = writer.write_all(response_str.as_bytes()).await;
                    }
                }
            } else {
                // For other commands, send immediately
                let guard = msg_sender.lock().await;
                if let Some(tx) = guard.as_ref() {
                    let _ = tx.send(Message::SocketCommand(cmd));
                }
                drop(guard);

                let response = serde_json::json!({
                    "status": "ok"
                });
                let response_str = serde_json::to_string(&response).unwrap() + "\n";
                let _ = writer.write_all(response_str.as_bytes()).await;
            }
        } else {
            // Send error response
            let response = serde_json::json!({
                "status": "error",
                "message": "Invalid JSON"
            });
            let response_str = serde_json::to_string(&response).unwrap() + "\n";
            let _ = writer.write_all(response_str.as_bytes()).await;
        }
    }
}

pub fn run_wizard() -> iced::Result {
    // Create channel for socket messages
    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
    let msg_sender = Arc::new(Mutex::new(Some(tx)));
    let receiver = Arc::new(Mutex::new(rx));

    // Create shared response sender
    let response_sender = Arc::new(Mutex::new(None));

    // Spawn socket server in tokio runtime
    let server_msg_sender = msg_sender.clone();
    let server_response_sender = response_sender.clone();
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(run_socket_server(server_msg_sender, server_response_sender));
    });

    // Run iced with subscription that listens to the channel
    let sub_receiver = receiver.clone();
    let window_response_sender = response_sender.clone();
    iced::application(
        WizardWindow::title,
        |state: &mut WizardWindow, message| {
            state.update(message);
            iced::Task::none()
        },
        WizardWindow::view
    )
    .subscription(move |_| {
        let rx = sub_receiver.clone();
        iced::Subscription::run_with_id(
            "socket-messages",
            futures::stream::unfold(rx, |rx| async move {
                let mut guard = rx.lock().await;
                guard.recv().await.map(|msg| (msg, rx.clone()))
            })
        )
    })
    .theme(WizardWindow::theme)
    .run_with(move || (WizardWindow::new(window_response_sender.clone()), iced::Task::none()))
}
