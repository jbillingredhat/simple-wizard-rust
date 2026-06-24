//! Main application entry point
//!
//! This module contains the run_wizard function that sets up and launches
//! the wizard application with the socket server.

use iced::{Element, Length};
use iced::widget::{container, column, row};
use std::sync::Arc;
use tokio::sync::Mutex;

use super::types::{Message, WizardWindow};
use super::socket::run_socket_server;

impl WizardWindow {
    pub fn view(&self) -> Element<'_, Message> {
        let info_panel = self.build_info_panel();
        let content_area = self.build_content_area();
        let progress_panel = self.build_progress_panel();

        let main_content = row![
            container(info_panel)
                .width(250)
                .height(Length::Fill)
                .padding(12),
            container(content_area)
                .width(Length::Fill)
                .height(Length::Fill)
                .padding(12),
        ]
        .height(Length::Fill);  // Make row expand to fill height

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
}

/// Run the wizard application
///
/// Sets up the socket server for external communication and launches the
/// iced GUI application.
///
/// # Arguments
/// * `socket_path` - Optional custom socket path (defaults to /tmp/simple-wizard.sock)
pub fn run_wizard(socket_path: Option<String>) -> iced::Result {
    // Create channel for socket messages
    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
    let msg_sender = Arc::new(Mutex::new(Some(tx)));
    let receiver = Arc::new(Mutex::new(rx));

    // Create shared response sender
    let response_sender = Arc::new(Mutex::new(None));

    // Spawn socket server in tokio runtime
    let server_msg_sender = msg_sender.clone();
    let server_response_sender = response_sender.clone();
    let socket_path_clone = socket_path.clone();
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(run_socket_server(server_msg_sender, server_response_sender, socket_path_clone));
    });

    // Run iced with subscription that listens to the channel
    let sub_receiver = receiver.clone();
    let window_response_sender = response_sender.clone();
    iced::application(
        WizardWindow::title,
        |state: &mut WizardWindow, message| {
            state.update(message)
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
