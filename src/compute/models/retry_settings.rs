use serde::{Deserialize, Serialize};

/// RetrySettings : Configuration for instance or task retry
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RetrySettings {
    /// Maximum number of total retries for the whole task
    pub max_total_retries: Option<i32>,
    /// Maximum number of retries for each instance
    pub max_per_instance_retries: Option<i32>,
}

impl RetrySettings {
    /// Configuration for instance or task retry
    pub const fn new() -> Self {
        Self {
            max_total_retries: None,
            max_per_instance_retries: None,
        }
    }
}
