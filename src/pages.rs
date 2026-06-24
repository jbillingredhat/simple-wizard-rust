use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageResponse {
    pub action: String,
    #[serde(flatten)]
    pub data: HashMap<String, String>,
}

// Validation presets
pub struct ValidationPresets;

impl ValidationPresets {
    pub fn get(preset: &str) -> Option<(&'static str, &'static str)> {
        match preset {
            "email" => Some((
                r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$",
                "Please enter a valid email address (e.g., user@example.com)",
            )),
            "url" => Some((
                r"^https?://[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}(/.*)?$",
                "Please enter a valid URL (e.g., https://example.com)",
            )),
            "ipv4" => Some((
                r"^((25[0-5]|(2[0-4]|1\d|[1-9]|)\d)\.?\b){4}$",
                "Please enter a valid IPv4 address (e.g., 192.168.1.1)",
            )),
            "port" => Some((
                r"^([1-9][0-9]{0,3}|[1-5][0-9]{4}|6[0-4][0-9]{3}|65[0-4][0-9]{2}|655[0-2][0-9]|6553[0-5])$",
                "Please enter a valid port number (1-65535)",
            )),
            "hostname" => Some((
                r"^[a-zA-Z0-9]([a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(\.[a-zA-Z0-9]([a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$",
                "Please enter a valid hostname (e.g., example.com)",
            )),
            "username" => Some((
                r"^[a-zA-Z0-9_-]{3,32}$",
                "Please enter a valid username (3-32 alphanumeric characters, dashes, or underscores)",
            )),
            "number" => Some((
                r"^-?\d+$",
                "Please enter a valid number",
            )),
            "positive_number" => Some((
                r"^\d+$",
                "Please enter a positive number",
            )),
            "alphanumeric" => Some((
                r"^[a-zA-Z0-9]+$",
                "Please enter only letters and numbers",
            )),
            _ => None,
        }
    }
}
