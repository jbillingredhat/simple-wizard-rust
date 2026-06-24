use serde::{Deserialize, Serialize};
use std::os::unix::net::UnixStream;
use std::io::{Read, Write};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Command {
    pub command: String,
    #[serde(flatten)]
    pub params: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub status: String,
    #[serde(flatten)]
    pub data: HashMap<String, serde_json::Value>,
}

pub struct WizardClient {
    socket_path: String,
}

impl WizardClient {
    pub fn new(socket_path: &str) -> Self {
        Self {
            socket_path: socket_path.to_string(),
        }
    }

    fn send_command(&self, command: Command) -> Result<Response, String> {
        let mut stream = UnixStream::connect(&self.socket_path)
            .map_err(|e| format!("Failed to connect: {}", e))?;

        let json = serde_json::to_string(&command)
            .map_err(|e| format!("Failed to serialize: {}", e))?;

        stream.write_all(json.as_bytes())
            .map_err(|e| format!("Failed to write: {}", e))?;
        stream.write_all(b"\n")
            .map_err(|e| format!("Failed to write newline: {}", e))?;

        let mut buffer = String::new();
        stream.read_to_string(&mut buffer)
            .map_err(|e| format!("Failed to read: {}", e))?;

        serde_json::from_str(&buffer)
            .map_err(|e| format!("Failed to deserialize: {}", e))
    }

    pub fn set_info(&self, title: Option<&str>, description: Option<&str>, help_text: Option<&str>) -> Result<Response, String> {
        let mut params = HashMap::new();
        if let Some(t) = title {
            params.insert("title".to_string(), serde_json::Value::String(t.to_string()));
        }
        if let Some(d) = description {
            params.insert("description".to_string(), serde_json::Value::String(d.to_string()));
        }
        if let Some(h) = help_text {
            params.insert("help_text".to_string(), serde_json::Value::String(h.to_string()));
        }

        self.send_command(Command {
            command: "set_info".to_string(),
            params,
        })
    }

    pub fn set_progress(&self, current: Option<i32>, total: Option<i32>, status: Option<&str>) -> Result<Response, String> {
        let mut params = HashMap::new();
        if let Some(c) = current {
            params.insert("current".to_string(), serde_json::Value::Number(c.into()));
        }
        if let Some(t) = total {
            params.insert("total".to_string(), serde_json::Value::Number(t.into()));
        }
        if let Some(s) = status {
            params.insert("status".to_string(), serde_json::Value::String(s.to_string()));
        }

        self.send_command(Command {
            command: "set_progress".to_string(),
            params,
        })
    }

    pub fn show_welcome(&self, title: &str, message: &str) -> Result<Response, String> {
        let mut page_params = HashMap::new();
        page_params.insert("title".to_string(), serde_json::Value::String(title.to_string()));
        page_params.insert("message".to_string(), serde_json::Value::String(message.to_string()));

        let mut params = HashMap::new();
        params.insert("page_type".to_string(), serde_json::Value::String("welcome".to_string()));
        params.insert("params".to_string(), serde_json::Value::Object(
            page_params.into_iter().map(|(k, v)| (k, v)).collect()
        ));

        self.send_command(Command {
            command: "show_page".to_string(),
            params,
        })
    }

    pub fn show_file(&self, title: &str, message: &str, default_path: &str) -> Result<Response, String> {
        let mut page_params = HashMap::new();
        page_params.insert("title".to_string(), serde_json::Value::String(title.to_string()));
        page_params.insert("message".to_string(), serde_json::Value::String(message.to_string()));
        page_params.insert("default_path".to_string(), serde_json::Value::String(default_path.to_string()));

        let mut params = HashMap::new();
        params.insert("page_type".to_string(), serde_json::Value::String("file".to_string()));
        params.insert("params".to_string(), serde_json::Value::Object(
            page_params.into_iter().map(|(k, v)| (k, v)).collect()
        ));

        self.send_command(Command {
            command: "show_page".to_string(),
            params,
        })
    }

    pub fn show_directory(&self, title: &str, message: &str, default_path: &str) -> Result<Response, String> {
        let mut page_params = HashMap::new();
        page_params.insert("title".to_string(), serde_json::Value::String(title.to_string()));
        page_params.insert("message".to_string(), serde_json::Value::String(message.to_string()));
        page_params.insert("default_path".to_string(), serde_json::Value::String(default_path.to_string()));

        let mut params = HashMap::new();
        params.insert("page_type".to_string(), serde_json::Value::String("directory".to_string()));
        params.insert("params".to_string(), serde_json::Value::Object(
            page_params.into_iter().map(|(k, v)| (k, v)).collect()
        ));

        self.send_command(Command {
            command: "show_page".to_string(),
            params,
        })
    }

    pub fn show_password(&self, title: &str, message: &str, confirm: bool, min_length: Option<usize>) -> Result<Response, String> {
        let mut page_params = HashMap::new();
        page_params.insert("title".to_string(), serde_json::Value::String(title.to_string()));
        page_params.insert("message".to_string(), serde_json::Value::String(message.to_string()));
        page_params.insert("confirm".to_string(), serde_json::Value::Bool(confirm));

        if let Some(len) = min_length {
            page_params.insert("min_length".to_string(), serde_json::Value::Number(len.into()));
        }

        let mut params = HashMap::new();
        params.insert("page_type".to_string(), serde_json::Value::String("password".to_string()));
        params.insert("params".to_string(), serde_json::Value::Object(
            page_params.into_iter().map(|(k, v)| (k, v)).collect()
        ));

        self.send_command(Command {
            command: "show_page".to_string(),
            params,
        })
    }

    pub fn show_question(&self, title: &str, message: &str, buttons: Vec<String>) -> Result<Response, String> {
        let mut page_params = HashMap::new();
        page_params.insert("title".to_string(), serde_json::Value::String(title.to_string()));
        page_params.insert("message".to_string(), serde_json::Value::String(message.to_string()));
        page_params.insert("buttons".to_string(), serde_json::Value::Array(
            buttons.into_iter().map(|b| serde_json::Value::String(b)).collect()
        ));

        let mut params = HashMap::new();
        params.insert("page_type".to_string(), serde_json::Value::String("question".to_string()));
        params.insert("params".to_string(), serde_json::Value::Object(
            page_params.into_iter().map(|(k, v)| (k, v)).collect()
        ));

        self.send_command(Command {
            command: "show_page".to_string(),
            params,
        })
    }

    pub fn show_text(&self, title: &str, message: &str, default_text: &str, placeholder: &str,
                     validate: Option<&str>, validation_message: Option<&str>) -> Result<Response, String> {
        let mut page_params = HashMap::new();
        page_params.insert("title".to_string(), serde_json::Value::String(title.to_string()));
        page_params.insert("message".to_string(), serde_json::Value::String(message.to_string()));
        page_params.insert("default_text".to_string(), serde_json::Value::String(default_text.to_string()));
        page_params.insert("placeholder".to_string(), serde_json::Value::String(placeholder.to_string()));

        if let Some(v) = validate {
            page_params.insert("validate".to_string(), serde_json::Value::String(v.to_string()));
        }
        if let Some(vm) = validation_message {
            page_params.insert("validation_message".to_string(), serde_json::Value::String(vm.to_string()));
        }

        let mut params = HashMap::new();
        params.insert("page_type".to_string(), serde_json::Value::String("text".to_string()));
        params.insert("params".to_string(), serde_json::Value::Object(
            page_params.into_iter().map(|(k, v)| (k, v)).collect()
        ));

        self.send_command(Command {
            command: "show_page".to_string(),
            params,
        })
    }

    pub fn show_warning(&self, title: &str, message: &str) -> Result<Response, String> {
        let mut page_params = HashMap::new();
        page_params.insert("title".to_string(), serde_json::Value::String(title.to_string()));
        page_params.insert("message".to_string(), serde_json::Value::String(message.to_string()));

        let mut params = HashMap::new();
        params.insert("page_type".to_string(), serde_json::Value::String("warning".to_string()));
        params.insert("params".to_string(), serde_json::Value::Object(
            page_params.into_iter().map(|(k, v)| (k, v)).collect()
        ));

        self.send_command(Command {
            command: "show_page".to_string(),
            params,
        })
    }

    pub fn show_error(&self, title: &str, message: &str) -> Result<Response, String> {
        let mut page_params = HashMap::new();
        page_params.insert("title".to_string(), serde_json::Value::String(title.to_string()));
        page_params.insert("message".to_string(), serde_json::Value::String(message.to_string()));

        let mut params = HashMap::new();
        params.insert("page_type".to_string(), serde_json::Value::String("error".to_string()));
        params.insert("params".to_string(), serde_json::Value::Object(
            page_params.into_iter().map(|(k, v)| (k, v)).collect()
        ));

        self.send_command(Command {
            command: "show_page".to_string(),
            params,
        })
    }

    pub fn show_complete(&self, title: &str, message: &str) -> Result<Response, String> {
        let mut page_params = HashMap::new();
        page_params.insert("title".to_string(), serde_json::Value::String(title.to_string()));
        page_params.insert("message".to_string(), serde_json::Value::String(message.to_string()));

        let mut params = HashMap::new();
        params.insert("page_type".to_string(), serde_json::Value::String("complete".to_string()));
        params.insert("params".to_string(), serde_json::Value::Object(
            page_params.into_iter().map(|(k, v)| (k, v)).collect()
        ));

        self.send_command(Command {
            command: "show_page".to_string(),
            params,
        })
    }

    pub fn append_log(&self, message: &str) -> Result<Response, String> {
        let mut params = HashMap::new();
        params.insert("message".to_string(), serde_json::Value::String(message.to_string()));

        self.send_command(Command {
            command: "append_log".to_string(),
            params,
        })
    }

    pub fn clear_log(&self) -> Result<Response, String> {
        self.send_command(Command {
            command: "clear_log".to_string(),
            params: HashMap::new(),
        })
    }

    pub fn quit(&self) -> Result<Response, String> {
        self.send_command(Command {
            command: "quit".to_string(),
            params: HashMap::new(),
        })
    }
}
