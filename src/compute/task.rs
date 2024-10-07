use std::str::FromStr;

use crate::compute::client::ComputeClient;
use crate::compute::models::{
    CompletedFrameOutput, Constants, HardwareConstraintVariant, Privileges, QError,
    QTaskStatusOutput, ResourcesBucket, RetrySettings, TaskCreationInput, TaskOutput,
    TaskUpdateInput,
};
use crate::compute::ComputeError;

use chrono::{DateTime, NaiveTime, TimeDelta, Utc};

pub enum ProfileOrPool {
    Profile(String),
    Pool(uuid::Uuid),
}

impl From<&str> for ProfileOrPool {
    fn from(profile: &str) -> Self {
        Self::Profile(profile.to_owned())
    }
}

impl From<uuid::Uuid> for ProfileOrPool {
    fn from(pool: uuid::Uuid) -> Self {
        Self::Pool(pool)
    }
}

pub enum InstancesOrRange {
    InstanceCount(i32),
    Range(String),
}

impl From<i32> for InstancesOrRange {
    fn from(instance_count: i32) -> Self {
        Self::InstanceCount(instance_count)
    }
}

impl From<&str> for InstancesOrRange {
    fn from(range: &str) -> Self {
        Self::Range(range.to_owned())
    }
}

pub enum State {
    Submitted,
    PartiallyDispatched,
    FullyDispatched,
    PartiallyExecuting,
    FullyExecuting,
    DownloadingResults,
    UploadingResults,
    Cancelled,
    Success,
    Failure,
    PendingDelete,
    PendingCancel,
}

impl From<&str> for State {
    fn from(state: &str) -> Self {
        match state {
            "Submitted" => Self::Submitted,
            "PartiallyDispatched" => Self::PartiallyDispatched,
            "FullyDispatched" => Self::FullyDispatched,
            "PartiallyExecuting" => Self::PartiallyExecuting,
            "FullyExecuting" => Self::FullyExecuting,
            "DownloadingResults" => Self::DownloadingResults,
            "UploadingResults" => Self::UploadingResults,
            "Cancelled" => Self::Cancelled,
            "Success" => Self::Success,
            "Failure" => Self::Failure,
            "PendingDelete" => Self::PendingDelete,
            "PendingCancel" => Self::PendingCancel,
            _ => Self::Failure,
        }
    }
}

impl State {
    pub const fn is_running_or_downloading(&self) -> bool {
        matches!(
            self,
            Self::Submitted
                | Self::PartiallyDispatched
                | Self::FullyDispatched
                | Self::PartiallyExecuting
                | Self::FullyExecuting
                | Self::DownloadingResults
                | Self::UploadingResults
        )
    }
}

/// High level wrapper around everything around tasks
/// This is returned by QarnotClient::create_task()
/// Use `.run()` to run task (asynchronously)
/// Use `.wait()` to wait for task to end (Success/Abort/Failure)
/// This wrapper contains both fields of task input and output, and non input
/// fields are updated when running run, wait, etc.
pub struct Task<'a> {
    compute_client: &'a ComputeClient,
    pub name: String,
    pub shortname: Option<String>,
    pub profile: Option<String>,
    pub pool_uuid: Option<uuid::Uuid>,
    pub job_uuid: Option<uuid::Uuid>,
    pub instance_count: Option<i32>,
    pub advanced_range: Option<String>,
    pub running_core_count: u32,
    pub running_instance_count: u32,
    pub resouce_buckets: Option<Vec<String>>,
    pub advanced_resource_buckets: Option<Vec<ResourcesBucket>>,
    pub result_bucket: Option<String>,
    pub constants: Option<Constants>,
    pub dependent_on: Option<Vec<uuid::Uuid>>,
    pub auto_update: bool,
    pub last_auto_update_state: bool,
    pub update_cache_time: TimeDelta,
    pub last_cache: DateTime<Utc>,
    pub constraints: Option<Constants>,
    pub labels: Option<std::collections::HashMap<String, String>>,
    pub state: Option<State>,
    pub uuid: Option<uuid::Uuid>,
    //pub snapshots: ???
    pub dirty: bool,
    pub rescount: i32,
    pub snapshot_whitelist: Option<String>,
    pub snapshot_blacklist: Option<String>,
    pub result_whitelist: Option<String>,
    pub result_blacklist: Option<String>,
    pub status: Option<QTaskStatusOutput>,
    pub completed_instances: Option<Vec<CompletedFrameOutput>>,
    pub tags: Option<Vec<String>>,
    pub errors: Option<Vec<QError>>,
    pub is_summary: bool,
    pub completion_time_to_live: Option<DateTime<Utc>>,
    pub auto_delete: bool,
    pub wait_for_pool_resources_synchronization: Option<bool>,
    pub previous_state: Option<String>,
    pub last_modified: Option<DateTime<Utc>>,
    pub snapshot_interval: Option<u32>,
    pub progress: Option<f32>,
    pub execution_time: Option<NaiveTime>,
    pub wall_time: Option<NaiveTime>,
    pub end_date: Option<DateTime<Utc>>,
    pub upload_results_on_cancellation: bool,
    pub hardware_constraints: Option<Vec<HardwareConstraintVariant>>,
    pub default_resources_cache_ttl_sec: Option<u32>,
    pub privileges: Option<Privileges>,
    pub retry_settings: Option<RetrySettings>,
}

impl<'a> Task<'a> {
    pub fn new(
        compute_client: &'a ComputeClient,
        name: &str,
        profile_pool: ProfileOrPool,
        shortname: Option<String>,
        instance_range: InstancesOrRange,
    ) -> Self {
        let mut instance_count: Option<i32> = None;
        let mut advanced_range: Option<String> = None;
        let mut profile: Option<String> = None;
        let mut pool_uuid: Option<uuid::Uuid> = None;
        match instance_range {
            InstancesOrRange::InstanceCount(n) => instance_count = Some(n),
            InstancesOrRange::Range(s) => advanced_range = Some(s),
        }
        match profile_pool {
            ProfileOrPool::Pool(p) => pool_uuid = Some(p),
            ProfileOrPool::Profile(p) => profile = Some(p),
        }
        Self {
            compute_client,
            name: name.to_owned(),
            shortname,
            profile,
            pool_uuid,
            job_uuid: None,
            instance_count,
            advanced_range,
            running_core_count: 0,
            running_instance_count: 0,
            resouce_buckets: None,
            advanced_resource_buckets: None,
            result_bucket: None,
            constants: None,
            dependent_on: None,
            auto_update: false,
            last_auto_update_state: false,
            update_cache_time: TimeDelta::seconds(5),
            last_cache: Utc::now(),
            constraints: None,
            labels: None,
            state: None,
            uuid: None,
            dirty: false,
            rescount: 0,
            snapshot_whitelist: None,
            snapshot_blacklist: None,
            result_whitelist: None,
            result_blacklist: None,
            status: None,
            completed_instances: None,
            tags: None,
            errors: None,
            is_summary: false,
            completion_time_to_live: None,
            auto_delete: false,
            wait_for_pool_resources_synchronization: None,
            previous_state: None,
            last_modified: None,
            snapshot_interval: None,
            progress: None,
            execution_time: None,
            wall_time: None,
            end_date: None,
            upload_results_on_cancellation: false,
            hardware_constraints: None,
            default_resources_cache_ttl_sec: None,
            privileges: None,
            retry_settings: None,
        }
    }

    /// Run task
    pub async fn run(&mut self) -> Result<(), ComputeError> {
        let input = TaskCreationInput {
            name: self.name.clone(),
            shortname: self.shortname.clone(),
            profile: self.profile.clone(),
            pool_uuid: self.pool_uuid,
            job_uuid: self.job_uuid,
            instance_count: self.instance_count,
            advanced_ranges: self.advanced_range.clone(),
            resource_buckets: self.resouce_buckets.clone(),
            advanced_resource_buckets: self.advanced_resource_buckets.clone(),
            result_bucket: self.result_bucket.clone(),
            constants: self.constants.clone(),
            forced_constants: None,
            constraints: self.constraints.clone(),
            hardware_constraints: self.hardware_constraints.clone(),
            secrets_access_rights: None,
            tags: self.tags.clone(),
            snapshot_whitelist: self.snapshot_whitelist.clone(),
            snapshot_blacklist: self.snapshot_blacklist.clone(),
            snapshot_bucket: None,
            snapshot_bucket_prefix: None,
            results_whitelist: self.result_whitelist.clone(),
            results_blacklist: self.result_blacklist.clone(),
            results_bucket: None,
            results_bucket_prefix: None,
            priority: None,
            dependencies: None,
            auto_delete_on_completion: Some(self.auto_delete),
            completion_time_to_live: None,
            wait_for_pool_resources_synchronization: None,
            upload_results_on_cancellation: Some(self.upload_results_on_cancellation),
            labels: self.labels.clone(),
            scheduling_type: None,
            targeted_reserved_machine_key: None,
            default_resources_cache_ttl_sec: None,
            privileges: self.privileges.clone(),
            retry_settings: self.retry_settings.clone(),
        };
        let res = self.compute_client.post_task(input).await?;
        self.uuid = res.uuid;
        if self.shortname.is_none() {
            self.shortname = res.uuid.map(|u| u.to_string());
        };
        Ok(())
    }

    /// Wait for task to finish
    pub async fn wait(&mut self) -> Result<(), ComputeError> {
        if let Some(_uuid) = self.uuid {
            while self.state.is_none()
                || self
                    .state
                    .as_ref()
                    .is_some_and(|s| s.is_running_or_downloading())
            {
                self.get_update(false).await?;
            }
            Ok(())
        } else {
            error!("No uuid, have you started the task ?");
            Err(ComputeError::Generic)
        }
    }

    /// Update current struct with values from a TaskOutput
    fn update_fields(&mut self, updated_task: TaskOutput) {
        if let Some(name) = updated_task.name {
            self.name = name;
        }
        self.shortname = updated_task.shortname;
        self.profile = updated_task.profile;
        self.pool_uuid = updated_task.pool_uuid;
        self.job_uuid = updated_task.job_uuid;
        self.instance_count = updated_task.instance_count;
        self.advanced_range = updated_task.advanced_ranges;
        self.wait_for_pool_resources_synchronization =
            updated_task.wait_for_pool_resources_synchronization;
        self.uuid = updated_task.uuid;
        self.state = updated_task.state.map(|s| State::from(s.as_str()));
        self.tags = updated_task.tags;
        if let Some(upload_res) = updated_task.upload_results_on_cancellation {
            self.upload_results_on_cancellation = upload_res;
        }
        self.previous_state = updated_task.previous_state;
        self.last_modified = updated_task
            .last_modified
            .map(|s| DateTime::<Utc>::from_str(s.as_str()).unwrap_or_default());
        self.progress = updated_task.progress;
        self.execution_time = updated_task
            .execution_time
            .map(|s| NaiveTime::parse_from_str(s.as_str(), "%H%M%S").unwrap_or_default());
        self.wall_time = updated_task
            .wall_time
            .map(|s| NaiveTime::parse_from_str(s.as_str(), "%H%M%S").unwrap_or_default());
        self.end_date = updated_task
            .end_date
            .map(|s| DateTime::<Utc>::from_str(s.as_str()).unwrap_or_default());
        self.labels = updated_task.labels;
        self.hardware_constraints = updated_task.hardware_constraints;
    }

    /// Update struct with changes from the API
    pub async fn get_update(&mut self, force_update: bool) -> Result<(), ComputeError> {
        if let Some(uuid) = self.uuid {
            if force_update || Utc::now() - self.last_cache >= self.update_cache_time {
                let res = self.compute_client.get_task_info(uuid).await?;
                self.update_fields(res);
                self.last_cache = Utc::now();
            }
        }
        Ok(())
    }

    /// Push changes to the struct to the compute API
    /// This results in PUT /task/{uuid}
    pub async fn commit(&self) -> Result<(), ComputeError> {
        if let Some(uuid) = self.uuid {
            let input = TaskUpdateInput {
                constants: self.constants.clone(),
                constraints: self.constraints.clone(),
                tags: self.tags.clone(),
            };
            self.compute_client.put_update_task(uuid, input).await
        } else {
            Ok(())
        }
    }

    /// Get current stdout of the task
    pub async fn stdout(&self) -> Result<String, ComputeError> {
        if let Some(uuid) = self.uuid {
            let res = self.compute_client.get_task_stdout(uuid).await?;
            Ok(res)
        } else {
            Ok(String::new())
        }
    }

    /// Get current stdout of the task
    pub async fn stderr(&self) -> Result<String, ComputeError> {
        if let Some(uuid) = self.uuid {
            let res = self.compute_client.get_task_stderr(uuid).await?;
            Ok(res)
        } else {
            Ok(String::new())
        }
    }
}
