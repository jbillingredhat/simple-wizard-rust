//! Socket server for external communication
//!
//! This module implements the Unix socket server that allows external processes
//! to communicate with the wizard GUI. It handles incoming commands and sends
//! responses back to clients.

use serde_json::Value;
use tokio::net::{UnixListener, UnixStream};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use std::sync::Arc;
use tokio::sync::{Mutex, oneshot};

use super::types::Message;

/// Runs the socket server that listens for external commands
///
/// The server creates a Unix socket at /tmp/simple-wizard.sock and accepts
/// incoming connections. Each connection is handled in a separate task.
pub async fn run_socket_server(
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

/// Handles a single socket connection
///
/// Reads a JSON command from the client, processes it, and sends back a response.
/// For show_page commands, waits for user interaction before responding.
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
