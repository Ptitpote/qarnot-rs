use crate::compute::models;
use serde::{Deserialize, Serialize};

/// `CompletedFrameOutput` : Information about the completed instance
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CompletedFrameOutput {
    /// List of the instance results
    pub results: Option<Vec<String>>,
    /// Id of the completed instance
    pub instance_id: Option<i32>,
    /// Actual life time of the instance
    pub wall_time_sec: Option<f32>,
    /// Execution time of the instance in seconds
    pub exec_time_sec: Option<f32>,
    /// Relative execution time
    #[serde(rename = "execTimeSecGHz")]
    pub exec_time_sec_ghz: Option<f32>,
    /// Maximum memory used in MB
    #[serde(rename = "peakMemoryMB")]
    pub peak_memory_mb: Option<i32>,
    /// State of the completed instance
    pub state: Option<String>,
    pub error: Option<models::QError>,
    /// Specification Key of the hardware used to compute the instance
    pub specification_key: Option<String>,
    /// Name of the cpu model used to compute the instance
    pub cpu_model: Option<String>,
    /// Number of cores of the cpu model
    pub core_count: Option<i32>,
    /// Clock ratio
    pub clock_ratio: Option<f64>,
    /// Average frequency (number of actions per second) during the execution (in GHz)
    #[serde(rename = "averageGHz")]
    pub average_ghz: Option<f32>,
    /// Number of execution attempts before completion
    pub execution_attempt_count: Option<i32>,
}
