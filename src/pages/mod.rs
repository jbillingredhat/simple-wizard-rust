/// Page types and validation for the wizard
///
/// This module contains all page type definitions and validation logic.
/// Each page type is defined in its own submodule for easier maintenance
/// and extension.

mod validation;

// Re-export the validation system
pub use validation::ValidationPresets;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Response from a page interaction
///
/// This is returned when the user completes interaction with a page.
/// The `action` field indicates what the user did (e.g., "next", "cancel", "finish")
/// and the `data` field contains any additional data (e.g., text input, file path).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageResponse {
    pub action: String,
    #[serde(flatten)]
    pub data: HashMap<String, String>,
}
