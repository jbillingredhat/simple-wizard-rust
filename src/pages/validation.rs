/// Validation presets for text input
///
/// This module provides pre-defined validation patterns for common input types.
/// Each preset includes a regex pattern and a user-friendly error message.
///
/// # Adding New Validation Presets
///
/// To add a new validation preset:
/// 1. Add a new match arm in `ValidationPresets::get()`
/// 2. Provide a regex pattern and error message
/// 3. Update the documentation in this file
/// 4. Update docs/API_COMPATIBILITY.md with the new preset
///
/// # Example
///
/// ```
/// use simple_wizard::pages::ValidationPresets;
///
/// if let Some((pattern, message)) = ValidationPresets::get("email") {
///     println!("Pattern: {}", pattern);
///     println!("Message: {}", message);
/// }
/// ```

pub struct ValidationPresets;

impl ValidationPresets {
    /// Get the regex pattern and error message for a validation preset
    ///
    /// # Arguments
    ///
    /// * `preset` - The name of the validation preset (e.g., "email", "url", "ipv4")
    ///
    /// # Returns
    ///
    /// * `Some((pattern, message))` - The regex pattern and error message
    /// * `None` - If the preset is not recognized
    ///
    /// # Available Presets
    ///
    /// - `email` - Email address validation (RFC 5322 subset)
    /// - `url` - HTTP/HTTPS URL validation
    /// - `ipv4` - IPv4 address validation
    /// - `port` - Port number validation (1-65535)
    /// - `hostname` - Hostname validation
    /// - `username` - Username validation (3-32 alphanumeric chars)
    /// - `number` - Any integer
    /// - `positive_number` - Positive integers only
    /// - `alphanumeric` - Letters and numbers only
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

    /// Get all available validation preset names
    ///
    /// This is useful for documentation and CLI help text.
    ///
    /// # Returns
    ///
    /// A vector of all available preset names
    pub fn available_presets() -> Vec<&'static str> {
        vec![
            "email",
            "url",
            "ipv4",
            "port",
            "hostname",
            "username",
            "number",
            "positive_number",
            "alphanumeric",
        ]
    }

    /// Check if a preset exists
    ///
    /// # Arguments
    ///
    /// * `preset` - The preset name to check
    ///
    /// # Returns
    ///
    /// `true` if the preset exists, `false` otherwise
    pub fn exists(preset: &str) -> bool {
        Self::get(preset).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_preset_exists() {
        assert!(ValidationPresets::exists("email"));
    }

    #[test]
    fn test_unknown_preset() {
        assert!(!ValidationPresets::exists("nonexistent"));
    }

    #[test]
    fn test_all_presets_available() {
        for preset in ValidationPresets::available_presets() {
            assert!(
                ValidationPresets::exists(preset),
                "Preset {} not available",
                preset
            );
        }
    }
}
