use serde::{Deserialize, Serialize};

/// QTaskStatusOutput : Detail of the task execution status
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QTaskStatusOutput {
    /// Date of the status update
    pub timestamp: Option<String>,
    /// Date of the last status update
    pub last_update_timestamp: Option<String>,
    /// Resources download progress.
    pub download_progress: Option<f32>,
    /// Execution progress.
    pub execution_progress: Option<f32>,
    /// Upload progress.
    pub upload_progress: Option<f32>,
    /// Number of running instances.
    pub instance_count: Option<i32>,
    /// Time of the resources download.
    pub download_time: Option<String>,
    /// Time of the resources download in seconds.
    pub download_time_sec: Option<f64>,
    /// Environment set up time, ie. the time needed to boot.
    pub environment_time: Option<String>,
    /// Time of the environment setup in seconds.
    pub environment_time_sec: Option<f64>,
    /// Execution time
    pub execution_time: Option<String>,
    /// Execution time in seconds
    pub execution_time_sec: Option<f64>,
    /// Time of the task execution with each cpu model
    pub execution_time_by_cpu_model: Option<Vec<CpuModelExecutionTimeOutput>>,
    /// Time of the task execution with each machine specification
    pub execution_time_by_machine_specification: Option<Vec<TimeByMachineSpecificationOutput>>,
    /// Time of the task execution with each machine specification
    pub execution_time_by_instance_id: Option<Vec<TimeByInstanceOutput>>,
    /// Relative Time (Ghz) of the task execution with each cpu model
    pub execution_time_ghz_by_cpu_model: Option<Vec<CpuModelExecutionTimeGhzOutput>>,
    /// Time of the results upload.
    pub upload_time: Option<String>,
    /// Time of the results upload in seconds.
    pub upload_time_sec: Option<f64>,
    /// Task's wall time.
    pub wall_time: Option<String>,
    /// Task's wall time in seconds.
    pub wall_time_sec: Option<f64>,
    /// Succeeded instances range.
    pub succeeded_range: Option<String>,
    /// Executed instances range, ie. the succeeded range plus the failed range.
    pub executed_range: Option<String>,
    /// Failed instances range, if there is any.
    pub failed_range: Option<String>,
    /// Cancelled instances range, if there is any.
    pub cancelled_range: Option<String>,
    /// Failed instances range without the cancelled range, if there is any.
    pub failed_only_range: Option<String>,
    /// Instances that have started at least once, used for minimum billing
    pub started_once_range: Option<String>,
    pub running_instances_info: Option<Box<QRunningInstancesInfoOutput>>,
}

impl QTaskStatusOutput {
    /// Detail of the task execution status
    pub const fn new() -> Self {
        Self {
            timestamp: None,
            last_update_timestamp: None,
            download_progress: None,
            execution_progress: None,
            upload_progress: None,
            instance_count: None,
            download_time: None,
            download_time_sec: None,
            environment_time: None,
            environment_time_sec: None,
            execution_time: None,
            execution_time_sec: None,
            execution_time_by_cpu_model: None,
            execution_time_by_machine_specification: None,
            execution_time_by_instance_id: None,
            execution_time_ghz_by_cpu_model: None,
            upload_time: None,
            upload_time_sec: None,
            wall_time: None,
            wall_time_sec: None,
            succeeded_range: None,
            executed_range: None,
            failed_range: None,
            cancelled_range: None,
            failed_only_range: None,
            started_once_range: None,
            running_instances_info: None,
        }
    }
}

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CpuModelExecutionTimeGhzOutput {
    /// Cpu model name
    pub model: Option<String>,
    /// Relative execution time
    pub time_ghz: Option<f64>,
    /// Clock ration
    pub clock_ratio: Option<f64>,
    /// Number of cores
    pub core: Option<u32>,
}

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CpuModelExecutionTimeOutput {
    /// Cpu model name
    pub model: Option<String>,
    /// Execution time
    pub time: Option<f64>,
    /// Number of cores
    pub core: Option<u32>,
}

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeByMachineSpecificationOutput {
    pub specification_key: Option<String>,
    pub time: Option<f64>,
}

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeByInstanceOutput {
    pub instance_id: Option<u32>,
    pub specification_key: Option<String>,
    pub site_uuid: Option<String>,
    pub time: Option<f64>,
    pub time_ghz: Option<f64>,
    pub clock_ration: Option<f64>,
}

/// QRunningInstancesInfoOutput : Description of the resources a job is currently executing on
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QRunningInstancesInfoOutput {
    /// List of all the Qarnot.Compute.Interface.QRunningInstanceInfoOutput each instance of the task is executing on
    pub per_running_instance_info: Option<Vec<QRunningInstanceInfoOutput>>,
    /// Time at which this information has been reported (last update)
    pub timestamp: Option<String>,
    /// Average frequency the CPUs executing this task are running at, in Ghz
    #[serde(rename = "averageFrequencyGHz")]
    pub average_frequency_ghz: Option<f32>,
    /// Maximum frequency the CPUs executing this task are running at, in Ghz
    #[serde(rename = "maxFrequencyGHz")]
    pub max_frequency_ghz: Option<f32>,
    /// Minimum frequency the CPUs executing this task are running at, inGhz
    #[serde(rename = "minFrequencyGHz")]
    pub min_frequency_ghz: Option<f32>,
    /// Average max frequency the CPUs could run at, in Ghz
    #[serde(rename = "averageMaxFrequencyGHz")]
    pub average_max_frequency_ghz: Option<f32>,
    /// Average cpu usage over all running CPUs
    pub average_cpu_usage: Option<f32>,
    /// Average ratio max frequency / actual frequency for CPUs executing the task. A good health status indicator for the cluster.
    pub cluster_power_indicator: Option<f32>,
    /// Average used memory / total memory over all CPUs executing the task
    pub average_memory_usage: Option<f32>,
    /// Average inbound network speed for all CPUs in Kbps
    pub average_network_in_kbps: Option<f32>,
    /// Average outbound network speed for all CPUs in Kbps
    pub average_network_out_kbps: Option<f32>,
    /// Total inbound network speed for all CPUs in Kbps
    pub total_network_in_kbps: Option<f32>,
    /// Total outbound network speed for all CPUs in Kbps
    pub total_network_out_kbps: Option<f32>,
    /// Core Information for each Cpu Model
    pub running_core_count_by_cpu_model: Option<Vec<CpuModelRunningCore>>,
}

/// QRunningInstanceInfoOutput : Details information about the execution of a running instance
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QRunningInstanceInfoOutput {
    /// All active forwardings
    pub active_forwards: Option<Vec<QTaskActiveForwardOutput>>,
    /// Vpn connections.
    pub vpn_connections: Option<Vec<TaskVpnConnectionOutput>>,
    pub phase: Option<QTaskExecutionPhaseOutput>,
    /// Instance Id executing on the CPU described here
    pub instance_id: Option<u32>,
    /// Maximum frequency the CPU can run at (in Ghz)
    #[serde(rename = "maxFrequencyGHz")]
    pub max_frequency_ghz: Option<f32>,
    /// Current frequency the CPU is running at
    #[serde(rename = "currentFrequencyGHz")]
    pub current_frequency_ghz: Option<f32>,
    /// Cpu usage
    pub cpu_usage: Option<f32>,
    /// Total size of the memory available for this CPU (in MegaBytes)
    #[serde(rename = "maxMemoryMB")]
    pub max_memory_mb: Option<u32>,
    /// Memory currently used by the instance
    #[serde(rename = "currentMemoryMB")]
    pub current_memory_mb: Option<u32>,
    /// Ratio of used memory
    pub memory_usage: Option<f32>,
    /// Current inbound network rate, in Kbps
    pub network_in_kbps: Option<f32>,
    /// Current outbound network rate, in Kbps
    pub network_out_kbps: Option<f32>,
    /// Instance's current progress
    pub progress: Option<f32>,
    /// Instance's current execution time in seconds
    pub execution_time_sec: Option<f32>,
    /// Instance's Current number of computing operations (in seconds Ghz)
    #[serde(rename = "executionTimeGHz")]
    pub execution_time_ghz: Option<f32>,
    /// Instance's hardware specification key.
    pub specification_key: Option<String>,
    /// Instance's cpu model.
    pub cpu_model: Option<String>,
    /// The core count.
    pub core_count: Option<u32>,
    /// The number of execution attempts.
    pub execution_attempt_count: Option<u32>,
    /// Clock ratio.
    pub clock_ratio: Option<f64>,
}

/// CpuModelRunningCore : Information about the cpu model's cores
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CpuModelRunningCore {
    /// Name of the Cpu model
    pub model: Option<String>,
    /// Number total of cores of the cpu
    pub core: Option<u32>,
    /// Number of running cores
    pub running_corecount: Option<u32>,
}

/// QTaskActiveForwardOutput : Forwarding information
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QTaskActiveForwardOutput {
    /// Port listening inside the payload
    pub application_port: Option<i32>,
    /// Port listening on the forwarder, forwarding traffic to PayloadPort inside the payload
    pub forwarder_port: Option<i32>,
    /// Forwarder host. Connecting to ForwarderHost:ForwarderPort will redirect to PayloadPort in the payload
    pub forwarder_host: Option<String>,
    /// Address the redirection is bound to on the forwarder server
    pub bind_address: Option<String>,
}

/// QTaskExecutionPhaseOutput : Possible execution state of the task
/// Possible execution state of the task
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum QTaskExecutionPhaseOutput {
    Download,
    Dispatch,
    Environment,
    Execution,
    Shutdown,
    Upload,
}

impl std::fmt::Display for QTaskExecutionPhaseOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Download => write!(f, "download"),
            Self::Dispatch => write!(f, "dispatch"),
            Self::Environment => write!(f, "environment"),
            Self::Execution => write!(f, "execution"),
            Self::Shutdown => write!(f, "shutdown"),
            Self::Upload => write!(f, "upload"),
        }
    }
}

impl Default for QTaskExecutionPhaseOutput {
    fn default() -> Self {
        Self::Download
    }
}

/// TaskVpnConnectionOutput : Vpn connection of the task
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskVpnConnectionOutput {
    /// Name of the vpn
    pub vpn_name: Option<String>,
    /// IP Address of the vpn connection
    pub node_ip_address_cidr: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn qtask_status_output_desrialize() {
        let example_str = r#"{
          "timestamp": "0001-01-01T00:00:00Z",
          "lastUpdateTimestamp": "0001-01-01T00:00:00Z",
          "downloadProgress": 100,
          "executionProgress": 0,
          "uploadProgress": 0,
          "instanceCount": 1,
          "downloadTime": "00:00:00",
          "downloadTimeSec": 0,
          "environmentTime": "00:10:38",
          "environmentTimeSec": 638,
          "executionTime": "00:00:00",
          "executionTimeSec": 0,
          "executionTimeByCpuModel":
          [
            {
              "model": "",
              "time": 0,
              "core": 0
            }
          ],
          "executionTimeByMachineSpecification":
          [
            {
              "specificationKey": "UNREGISTER_MODEL",
              "time": 0
            }
          ],
          "executionTimeByInstanceId": null,
          "executionTimeGhzByCpuModel":
          [
            {
              "model": "",
              "timeGhz": 0,
              "clockRatio": 0,
              "core": 0
            }
          ],
          "uploadTime": "00:00:00",
          "uploadTimeSec": 0,
          "wallTime": "00:11:06",
          "wallTimeSec": 666,
          "succeededRange": "",
          "executedRange": "",
          "failedRange": "",
          "cancelledRange": "",
          "failedOnlyRange": "",
          "startedOnceRange": "0",
          "runningInstancesInfo":
          {
            "perRunningInstanceInfo":
            [
              {
                "activeForwards": [],
                "vpnConnections": [],
                "phase": "execution",
                "instanceId": 0,
                "maxFrequencyGHz": 0,
                "currentFrequencyGHz": 0,
                "cpuUsage": 0,
                "maxMemoryMB": 0,
                "currentMemoryMB": 0,
                "memoryUsage": 0,
                "networkInKbps": 0,
                "networkOutKbps": 0,
                "progress": 0,
                "executionTimeSec": 0,
                "executionTimeGHz": 0,
                "specificationKey": "",
                "cpuModel": "",
                "coreCount": 0,
                "executionAttemptCount": 6,
                "clockRatio": 0
              }
            ],
            "snapshotResults": [],
            "timestamp": "0001-01-01T00:00:00Z",
            "averageFrequencyGHz": 0,
            "maxFrequencyGHz": 0,
            "minFrequencyGHz": 0,
            "averageMaxFrequencyGHz": 0,
            "averageCpuUsage": 0,
            "clusterPowerIndicator": 1,
            "averageMemoryUsage": 0,
            "averageNetworkInKbps": 0,
            "averageNetworkOutKbps": 0,
            "totalNetworkInKbps": 0,
            "totalNetworkOutKbps": 0,
            "runningCoreCountByCpuModel": []
          }
        }"#;
        let deser = serde_json::from_str::<QTaskStatusOutput>(example_str);
        assert!(deser.is_ok(), "{:?}", deser);
    }
}
