use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct QError {
    /// Error code
    pub code: Option<String>,
    /// Error message
    pub message: Option<String>,
    /// Error debug hints
    pub debug: Option<String>,
}
