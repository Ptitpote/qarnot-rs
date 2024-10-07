use crate::compute::models::{
    CompletedFrameOutput, Constants, DependencyInput, ForcedConstant, HardwareConstraintVariant,
    Privileges, QError, QTaskStatusOutput, ResourcesBucket, RetrySettings, SchedulingClass,
    SecretsAccessRights,
};
use serde::{Deserialize, Serialize};

/// `TaskCreationInput` : Input of a task creation request
#[derive(Clone, Default, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskCreationInput {
    /// Given Name of the task.  <br />Should be less than 2048 characters.
    pub name: String,
    /// Shortname of the task (default is its Uuid).
    /// <br />Should be <b>unique</b>.
    /// > Available characters (all alphanumeric + hyphen): ```a-z```, ```A-Z```, ```0-9``` and ```-```.
    /// > Should start with a letter or number and should not end with an hyphen (```-```).
    pub shortname: Option<String>,
    /// Profile used by the task  <br />Should be one of the available profiles for the user.
    /// <br />Should not be set if the task is using a pool
    pub profile: Option<String>,
    /// Uuid of the task's pool.
    /// <br />Should be the uuid of an open pool.
    /// <br />Should not be set if the task part of a job.
    /// If the job uses a pool, the task will automatically inherit it.
    pub pool_uuid: Option<uuid::Uuid>,
    /// Uuid of the task's job.  <br />Should be the uuid of an active job.
    pub job_uuid: Option<uuid::Uuid>,
    /// Number of task instances.
    /// <br />For a more complex definition of instance ids, use ```AdvancedRanges``` instead.
    /// <br />The use of one of ```InstanceCount``` or ```AdvancedRanges``` is required.
    /// <br />Maximum number of instances: 2048.
    pub instance_count: Option<i32>,
    /// Range of task instances, represented by a comma-separated list of specific id values or continuous ranges defined by hyphens (-).
    /// <br />(Optional - override `instance_count`) Specify an advanced range instead of a ```InstanceCount```.
    /// <br />The use of one of ```InstanceCount``` or ```AdvancedRanges``` is required.
    /// <br />Maximum number of instances: 2048.
    pub advanced_ranges: Option<String>,
    /// Names of the buckets containing resources for the task.
    /// <br /> The buckets should already exist in the user storage account.
    /// <br />Should not be set with ```AdvancedResourceBuckets```.
    /// <br />The use of either ```ResourceBuckets``` or ```AdvancedResourceBuckets``` is required.
    /// <br />Maximum number of buckets: 100.
    pub resource_buckets: Option<Vec<String>>,
    /// Names of the buckets containing resources for the task, with custom prefix.
    /// <br /> The buckets should already exist in the user storage account.
    /// <br />Should not be set with ```ResourceBuckets```.
    /// <br />The use of either ```ResourceBuckets``` or ```AdvancedResourceBuckets``` is required.
    /// <br />Maximum number of buckets: 100.
    pub advanced_resource_buckets: Option<Vec<ResourcesBucket>>,
    /// Name of the bucket containing results of the task.
    /// <br />If the bucket does not exist in the user storage account, it will be created.
    pub result_bucket: Option<String>,
    /// List of constants for the task.
    /// <br />They are used to configure the profiles and can be overridden to change its parameters.
    /// New constants can also be added to use during the Task execution.
    /// <br />Allowed characters for the constants' keys: ```a-z```, ```A-Z```, ```0-9```, ```_```, ```-```, ```.```<br />
    /// The constants' keys and values should not be null.
    /// <br />Maximum number of constants is 150. Maximum length of the key and value: 2048 characters.
    /// <br />For secret constants with write-only rights, use the prefix ```'QARNOT_SECRET__'```
    pub constants: Option<Constants>,
    /// List of constants to be overriden for the task.
    /// <br />This is expected to be used in a development context and requires specific permissions.
    /// <br />The same restrictions that apply to regular constants also apply here.
    pub forced_constants: Option<Vec<ForcedConstant>>,
    /// List of constraints for the task (can be set only by Admin users)
    pub constraints: Option<Constants>,
    /// Constraints applied to hardware for executing the task.  <br />Cannot set new hardware constraints for a task in a pool.
    pub hardware_constraints: Option<Vec<HardwareConstraintVariant>>,
    /// Describe secrets the task or pool will have access to when running.
    pub secrets_access_rights: Option<SecretsAccessRights>,
    /// (Optional) List of tags  <br />Should be less than 10.
    /// <br />The tags values should not be null and should be less than 512 characters.
    pub tags: Option<Vec<String>>,
    /// (Optional) Regex of whitelisted files for snapshots  <br />Should be less than 2048 characters.
    /// <br />You can use the values of task constants in the regex using ```${CONSTANT_KEY}```
    pub snapshot_whitelist: Option<String>,
    /// (Optional) Regex of blacklisted files for snapshots  <br />Should be less than 2048 characters.
    /// <br />You can use the values of task constants in the regex using ```${CONSTANT_KEY}```
    pub snapshot_blacklist: Option<String>,
    /// Name of the snapshot bucket
    pub snapshot_bucket: Option<String>,
    /// Prefix for the snapshot bucket
    pub snapshot_bucket_prefix: Option<String>,
    /// (Optional) Regex of whitelisted files for results  <br />Should be less than 2048 characters.
    /// <br />You can use the values of task constants in the regex using ```${CONSTANT_KEY}```
    pub results_whitelist: Option<String>,
    /// (Optional) Regex of blacklisted files for results  <br />Should be less than 2048 characters.
    /// <br />You can use the values of task constants in the regex using ```${CONSTANT_KEY}```
    pub results_blacklist: Option<String>,
    /// Name of the buckets used for results
    pub results_bucket: Option<String>,
    /// Prefix to add to bucket for results
    pub results_bucket_prefix: Option<String>,
    /// Priority of the task (can be set only by Admin users)
    pub priority: Option<i32>,
    pub dependencies: Option<DependencyInput>,
    /// Whether the task should be deleted if completed and the task quota is reached
    pub auto_delete_on_completion: Option<bool>,
    /// Task life time limit
    pub completion_time_to_live: Option<String>,
    /// For in-pool tasks, whether to consider pool resources synchronization as a barrier for execution
    pub wait_for_pool_resources_synchronization: Option<bool>,
    /// Whether the results should be upload if the task is cancelled
    pub upload_results_on_cancellation: Option<bool>,
    /// Task labels : arbitrary key / value pairs attached  to the task in order to find it more easily.  <br />They do not affect the execution of the task.
    pub labels: Option<std::collections::HashMap<String, String>>,
    pub scheduling_type: Option<SchedulingClass>,
    /// The key of the reserved machine the task should be dispatch on.  To use with ```\"reserved\"``` `scheduling_type`
    pub targeted_reserved_machine_key: Option<String>,
    /// The default TTL value for all the task resources cache.  TTL is 7776000s by default.
    pub default_resources_cache_ttl_sec: Option<i32>,
    pub privileges: Option<Privileges>,
    pub retry_settings: Option<RetrySettings>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TaskOutput {
    /// (Optional) Errors
    pub errors: Option<Vec<QError>>,
    /// Names of the buckets containing resources for the task
    pub resource_buckets: Option<Vec<String>>,
    pub advanced_resource_buckets: Option<Vec<ResourcesBucket>>,
    pub result_bucket: Option<String>,
    pub completed_instances: Option<Vec<CompletedFrameOutput>>,
    pub status: Option<QTaskStatusOutput>,
    pub snapshot_interval: Option<u32>,
    pub results_count: Option<u32>,
    /// List of constants for the task.  All secret constants with the prefix \"QARNOT_SECRET__\" will display \"[SECRET]\" value.
    pub constants: Option<Constants>,

    pub secrets_access_rights: Option<SecretsAccessRights>,
    /// (Optional) List of tags
    pub tags: Option<Vec<String>>,
    /// (Optional) Regex of whitelisted files for snapshots
    pub snapshot_whitelist: Option<String>,
    /// (Optional) Regex of blacklisted files for snapshots
    pub snapshot_blacklist: Option<String>,
    /// Name of the snapshot bucket
    pub snapshot_bucket: Option<String>,
    /// Prefix for the snapshot bucket
    pub snapshot_bucket_prefix: Option<String>,
    /// (Optional) Regex of whitelisted files for results
    pub results_whitelist: Option<String>,
    /// (Optional) Regex of blacklisted files for results
    pub results_blacklist: Option<String>,
    /// Name of the buckets used for results
    pub results_bucket: Option<String>,
    /// Prefix to add to bucket for results
    pub results_bucket_prefix: Option<String>,
    /// Whether the results should be upload if the task is cancelled
    pub upload_results_on_cancellation: Option<bool>,

    pub dependencies: Option<DependencyOutput>,
    /// Whether the task should be deleted if the task quota is reached and the task is completed
    pub auto_delete_on_completion: Option<bool>,
    /// Task life time limit
    pub completion_time_to_live: Option<String>,
    /// Constraints applied to hardware for executing the task
    pub hardware_constraints: Option<Vec<HardwareConstraintVariant>>,
    /// Task labels : arbitrary key / value pairs attached  to the task in order to find it more easily.
    pub labels: Option<std::collections::HashMap<String, String>>,

    pub scheduling_type: Option<SchedulingClass>,
    /// The key of the reserved machine the user wanted the task to be dispatched on.
    pub targeted_reserved_machine_key: Option<String>,

    pub privileges: Option<Privileges>,

    pub retry_settings: Option<RetrySettings>,
    /// Uuid of the task
    pub uuid: Option<uuid::Uuid>,
    /// Given Name of the task
    pub name: Option<String>,
    /// Shortname of the task
    pub shortname: Option<String>,
    /// Profile used by the task
    pub profile: Option<String>,
    /// Uuid of the task's pool
    pub pool_uuid: Option<uuid::Uuid>,
    /// Uuid of the task's job
    pub job_uuid: Option<uuid::Uuid>,
    /// Progression of the task execution
    pub progress: Option<f32>,
    /// Number of task instances that are currently running
    pub running_instance_count: Option<i32>,
    /// Number of cores currently used byt task instances
    pub running_core_count: Option<i32>,
    /// Current execution time
    pub execution_time: Option<String>,
    /// Limit execution time for the task
    pub wall_time: Option<String>,
    /// Current state of the task  <br>Can be: Submitted, PartiallyDispatched, FullyDispatched, PartiallyExecuting,  FullyExecuting, UploadingResults, Cancelled, Success, Failure, PendingDelete or PendingCancel
    pub state: Option<String>,
    /// Previous state of the task
    pub previous_state: Option<String>,
    /// Number of task instances
    pub instance_count: Option<i32>,
    /// Range of task instances  <br>(Optional - replace `instance_count`) Specify an advanced range instead of a InstanceCount
    pub advanced_ranges: Option<String>,
    /// Date of the last state transition
    pub state_transition_time: Option<String>,
    /// Date of the previous state transition
    pub previous_state_transition_time: Option<String>,
    /// Date of the last modification
    pub last_modified: Option<String>,
    /// Date of the task creation (UTC ISO 8601)
    pub creation_date: Option<String>,
    /// Date of the task end
    pub end_date: Option<String>,
    /// For in-pool tasks, whether to consider pool resources synchronization as a barrier for execution
    pub wait_for_pool_resources_synchronization: Option<bool>,
}

/// `TaskSummaryOutput` : Output of the Get request for task summaries
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct TaskSummaryOutput {
    /// Uuid of the task
    pub uuid: Option<uuid::Uuid>,
    /// Given Name of the task
    pub name: Option<String>,
    /// Shortname of the task
    pub shortname: Option<String>,
    /// Profile used by the task
    pub profile: Option<String>,
    /// Uuid of the task's pool
    pub pool_uuid: Option<uuid::Uuid>,
    /// Uuid of the task's job
    pub job_uuid: Option<uuid::Uuid>,
    /// Progression of the task execution
    pub progress: Option<f32>,
    /// Number of task instances that are currently running
    pub running_instance_count: Option<i32>,
    /// Number of cores currently used byt task instances
    pub running_core_count: Option<i32>,
    /// Current execution time
    pub execution_time: Option<String>,
    /// Limit execution time for the task
    pub wall_time: Option<String>,
    /// Current state of the task  <br>Can be: Submitted, PartiallyDispatched, FullyDispatched, PartiallyExecuting,  FullyExecuting, UploadingResults, Cancelled, Success, Failure, PendingDelete or PendingCancel
    pub state: Option<String>,
    /// Previous state of the task
    pub previous_state: Option<String>,
    /// Number of task instances
    pub instance_count: Option<i32>,
    /// Range of task instances  <br>(Optional - replace InstanceCount) Specify an advanced range instead of a InstanceCount
    pub advanced_ranges: Option<String>,
    /// Date of the last state transition
    pub state_transition_time: Option<String>,
    /// Date of the previous state transition
    pub previous_state_transition_time: Option<String>,
    /// Date of the last modification
    pub last_modified: Option<String>,
    /// Date of the task creation (UTC ISO 8601)
    pub creation_date: Option<String>,
    /// Date of the task end
    pub end_date: Option<String>,
    /// For in-pool tasks, whether to consider pool resources synchronization as a barrier for execution
    pub wait_for_pool_resources_synchronization: Option<bool>,
}

/// `DependencyOutput` : Initial task dependency
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DependencyOutput {
    /// List of tasks' uuids the task depends on.  The task will start only if these tasks are completed
    pub depends_on: Option<Vec<uuid::Uuid>>,
}

/// `TaskRedoInput` : Configuration of the fields to change in the retry/recover/resume of a task
#[derive(Clone, Default, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskRedoInput {
    /// Given Name of the cloned task. Same as the original by default.  <br />Should be less than 2048 characters.
    pub name: Option<String>,
    /// Names of the buckets containing resources for the task.  <br /> The buckets should already exist in the user storage account.  <br />If ```AdvancedResourceBuckets``` is set its value will be used instead.  <br />Maximum number of buckets: 100.
    pub resource_buckets: Option<Vec<String>>,
    /// Names of the buckets containing resources for the task, with custom prefix.  <br /> The buckets should already exist in the user storage account.  <br />Will override the value of ```ResourceBuckets```.  <br />Maximum number of buckets: 100.
    pub advanced_resource_buckets: Option<Vec<ResourcesBucket>>,
    /// Shortname of the task (default is its Uuid).  <br />Should be <b>unique</b>.  >Available characters (all alphanumeric + hyphen): ```a-z```, ```A-Z```, ```0-9``` and ```-```.  Should start with a letter or number and should not end with an hyphen (```-```).
    pub shortname: Option<String>,
    /// Profile used by the task  <br />Should be one of the available profiles for the user.  <br />Should not be set if the task is using a pool
    pub profile: Option<String>,
    /// Uuid of the task's job.  <br />Should be the uuid of an active job.
    pub job_uuid: Option<uuid::Uuid>,
    /// Name of the bucket containing results of the task.  <br />If the bucket does not exist in the user storage account, it will be created.
    pub result_bucket: Option<String>,
    /// List of constants for the task.  <br />They are used to configure the profiles and can be overridden to change its parameters.  New constants can also be added to use during the Task execution.  <br />Allowed characters for the constants' keys: ```a-z```, ```A-Z```, ```0-9```, ```_```, ```-```, ```.```<br />The constants' keys and values should not be null.  <br />Maximum number of constants is 150. Maximum length of the key and value: 2048 characters.  <br />For secret constants with write-only rights, use the prefix ```'QARNOT_SECRET__'```
    pub constants: Option<Constants>,
    /// List of constants to be overriden for the task.  <br />This is expected to be used in a development context and requires specific permissions.  <br />The same restrictions that apply to regular constants also apply here.
    pub forced_constants: Option<Vec<ForcedConstant>>,
    /// List of constraints for the task (can be set only by Admin users)
    pub constraints: Option<Constants>,
    /// Constraints applied to hardware for executing the task.  <br />Cannot set new hardware constraints for a task in a pool.
    pub hardware_constraints: Option<Vec<HardwareConstraintVariant>>,
    pub secrets_access_rights: Option<SecretsAccessRights>,
    /// (Optional) List of tags  <br />Should be less than 10.  <br />The tags values should not be null and should be less than 512 characters.
    pub tags: Option<Vec<String>>,
    /// (Optional) Regex of whitelisted files for snapshots  <br />Should be less than 2048 characters.  <br />You can use the values of task constants in the regex using ```${CONSTANT_KEY}```
    pub snapshot_whitelist: Option<String>,
    /// (Optional) Regex of blacklisted files for snapshots  <br />Should be less than 2048 characters.  <br />You can use the values of task constants in the regex using ```${CONSTANT_KEY}```
    pub snapshot_blacklist: Option<String>,
    /// Name of the snapshot bucket
    pub snapshot_bucket: Option<String>,
    /// Prefix for the snapshot bucket
    pub snapshot_bucket_prefix: Option<String>,
    /// (Optional) Regex of whitelisted files for results  <br />Should be less than 2048 characters.  <br />You can use the values of task constants in the regex using ```${CONSTANT_KEY}```
    pub results_whitelist: Option<String>,
    /// (Optional) Regex of blacklisted files for results  <br />Should be less than 2048 characters.  <br />You can use the values of task constants in the regex using ```${CONSTANT_KEY}```
    pub results_blacklist: Option<String>,
    /// Name of the buckets used for results
    pub results_bucket: Option<String>,
    /// Prefix to add to bucket for results
    pub results_bucket_prefix: Option<String>,
    /// Priority of the task (can be set only by Admin users)
    pub priority: Option<i32>,
    pub dependencies: Option<DependencyInput>,
    /// Whether the task should be deleted if completed and the task quota is reached
    pub auto_delete_on_completion: Option<bool>,
    /// Task life time limit
    pub completion_time_to_live: Option<String>,
    /// For in-pool tasks, whether to consider pool resources synchronization as a barrier for execution
    pub wait_for_pool_resources_synchronization: Option<bool>,
    /// Whether the results should be upload if the task is cancelled
    pub upload_results_on_cancellation: Option<bool>,
    /// Task labels : arbitrary key / value pairs attached  to the task in order to find it more easily.  <br />They do not affect the execution of the task.
    pub labels: Option<std::collections::HashMap<String, String>>,
    pub scheduling_type: Option<SchedulingClass>,
    /// The key of the reserved machine the task should be dispatch on.  To use with ```\"reserved\"``` SchedulingType
    pub targeted_reserved_machine_key: Option<String>,
    /// The default TTL value for all the task resources cache.  TTL is 7776000s by default.
    pub default_resources_cache_ttl_sec: Option<i32>,
    pub privileges: Option<Privileges>,
    pub retry_settings: Option<RetrySettings>,
}

/// `TaskCloneInput` : Configuration of the fields to change in the cloned task
#[derive(Clone, Default, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskCloneInput {
    /// Given Name of the cloned task.
    /// Same as the original by default.
    /// <br />Should be less than 2048 characters.
    pub name: Option<String>,
    /// Number of task instances.
    /// <br />For a more complex definition of instance ids, use ```AdvancedRanges``` instead.
    /// <br />If ```AdvancedRanges``` is set its value will be used instead.
    /// <br />Maximum number of instances: 2048.
    pub instance_count: Option<i32>,
    /// Range of task instances, represented by a comma-separated list of specific id values or continuous ranges defined by hyphens (-).
    /// <br />Will override the value of ```InstanceCount```.
    /// <br />Maximum number of instances: 2048.
    pub advanced_ranges: Option<String>,
    /// Names of the buckets containing resources for the task.
    /// <br /> The buckets should already exist in the user storage account.
    /// <br />If ```AdvancedResourceBuckets``` is set its value will be used instead.
    /// <br />Maximum number of buckets: 100.
    pub resource_buckets: Option<Vec<String>>,
    /// Names of the buckets containing resources for the task, with custom prefix.
    /// <br /> The buckets should already exist in the user storage account.
    /// <br />Will override the value of ```ResourceBuckets```.
    /// <br />Maximum number of buckets: 100.
    pub advanced_resource_buckets: Option<Vec<ResourcesBucket>>,
    /// Shortname of the task (default is its Uuid).
    /// <br />Should be <b>unique</b>.
    /// > Available characters (all alphanumeric + hyphen): ```a-z```, ```A-Z```, ```0-9``` and ```-```.
    /// > Should start with a letter or number and should not end with an hyphen (```-```).
    pub shortname: Option<String>,
    /// Profile used by the task  <br />Should be one of the available profiles for the user.
    /// <br />Should not be set if the task is using a pool
    pub profile: Option<String>,
    /// Uuid of the task's job.
    /// <br />Should be the uuid of an active job.
    pub job_uuid: Option<uuid::Uuid>,
    /// Name of the bucket containing results of the task.
    /// <br />If the bucket does not exist in the user storage account, it will be created.
    pub result_bucket: Option<String>,
    /// List of constants for the task.
    /// <br />They are used to configure the profiles and can be overridden to change its parameters.
    /// New constants can also be added to use during the Task execution.
    /// <br />Allowed characters for the constants' keys: ```a-z```, ```A-Z```, ```0-9```, ```_```, ```-```, ```.```<br />
    /// The constants' keys and values should not be null.
    /// <br />Maximum number of constants is 150.
    /// Maximum length of the key and value: 2048 characters.
    /// <br />For secret constants with write-only rights, use the prefix ```'QARNOT_SECRET__'```
    pub constants: Option<Constants>,
    /// List of constants to be overriden for the task.
    /// <br />This is expected to be used in a development context and requires specific permissions.
    /// <br />The same restrictions that apply to regular constants also apply here.
    pub forced_constants: Option<Vec<ForcedConstant>>,
    /// List of constraints for the task (can be set only by Admin users)
    pub constraints: Option<Constants>,
    /// Constraints applied to hardware for executing the task.
    /// <br />Cannot set new hardware constraints for a task in a pool.
    pub hardware_constraints: Option<Vec<HardwareConstraintVariant>>,
    pub secrets_access_rights: Option<Box<SecretsAccessRights>>,
    /// (Optional) List of tags  <br />Should be less than 10.
    /// <br />The tags values should not be null and should be less than 512 characters.
    pub tags: Option<Vec<String>>,
    /// (Optional) Regex of whitelisted files for snapshots  <br />Should be less than 2048 characters.
    /// <br />You can use the values of task constants in the regex using ```${CONSTANT_KEY}```
    pub snapshot_whitelist: Option<String>,
    /// (Optional) Regex of blacklisted files for snapshots  <br />Should be less than 2048 characters.
    /// <br />You can use the values of task constants in the regex using ```${CONSTANT_KEY}```
    pub snapshot_blacklist: Option<String>,
    /// Name of the snapshot bucket
    pub snapshot_bucket: Option<String>,
    /// Prefix for the snapshot bucket
    pub snapshot_bucket_prefix: Option<String>,
    /// (Optional) Regex of whitelisted files for results  <br />Should be less than 2048 characters.
    /// <br />You can use the values of task constants in the regex using ```${CONSTANT_KEY}```
    pub results_whitelist: Option<String>,
    /// (Optional) Regex of blacklisted files for results  <br />Should be less than 2048 characters.
    /// <br />You can use the values of task constants in the regex using ```${CONSTANT_KEY}```
    pub results_blacklist: Option<String>,
    /// Name of the buckets used for results
    pub results_bucket: Option<String>,
    /// Prefix to add to bucket for results
    pub results_bucket_prefix: Option<String>,
    /// Priority of the task (can be set only by Admin users)
    pub priority: Option<i32>,
    pub dependencies: Option<Box<DependencyInput>>,
    /// Whether the task should be deleted if completed and the task quota is reached
    pub auto_delete_on_completion: Option<bool>,
    /// Task life time limit
    pub completion_time_to_live: Option<String>,
    /// For in-pool tasks, whether to consider pool resources synchronization as a barrier for execution
    pub wait_for_pool_resources_synchronization: Option<bool>,
    /// Whether the results should be upload if the task is cancelled
    pub upload_results_on_cancellation: Option<bool>,
    /// Task labels : arbitrary key / value pairs attached to the task in order to find it more easily.
    /// <br />They do not affect the execution of the task.
    pub labels: Option<std::collections::HashMap<String, String>>,
    pub scheduling_type: Option<SchedulingClass>,
    /// The key of the reserved machine the task should be dispatch on.
    /// To use with ```\"reserved\"``` SchedulingType
    pub targeted_reserved_machine_key: Option<String>,
    /// The default TTL value for all the task resources cache.
    /// TTL is 7776000s by default.
    pub default_resources_cache_ttl_sec: Option<i32>,
    pub privileges: Option<Box<Privileges>>,
    pub retry_settings: Option<Box<RetrySettings>>,
}

/// `TaskUpdateInput` : Fields of the task to update
#[derive(Clone, Default, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskUpdateInput {
    /// List of new constants for the pool
    pub constants: Option<Constants>,
    /// List of new constraints for the pool (only available for Admin user)
    pub constraints: Option<Constants>,
    /// List of new tags for the pool
    pub tags: Option<Vec<String>>,
}

//XXX IMPLEMENATIONS

impl TaskCreationInput {
    /// Input of a task creation request
    pub const fn new(name: String) -> Self {
        Self {
            name,
            shortname: None,
            profile: None,
            pool_uuid: None,
            job_uuid: None,
            instance_count: None,
            advanced_ranges: None,
            resource_buckets: None,
            advanced_resource_buckets: None,
            result_bucket: None,
            constants: None,
            forced_constants: None,
            constraints: None,
            hardware_constraints: None,
            secrets_access_rights: None,
            tags: None,
            snapshot_whitelist: None,
            snapshot_blacklist: None,
            snapshot_bucket: None,
            snapshot_bucket_prefix: None,
            results_whitelist: None,
            results_blacklist: None,
            results_bucket: None,
            results_bucket_prefix: None,
            priority: None,
            dependencies: None,
            auto_delete_on_completion: None,
            completion_time_to_live: None,
            wait_for_pool_resources_synchronization: None,
            upload_results_on_cancellation: None,
            labels: None,
            scheduling_type: None,
            targeted_reserved_machine_key: None,
            default_resources_cache_ttl_sec: None,
            privileges: None,
            retry_settings: None,
        }
    }
}

impl TaskSummaryOutput {
    /// Output of the Get request for task summaries
    pub const fn new() -> Self {
        Self {
            uuid: None,
            name: None,
            shortname: None,
            profile: None,
            pool_uuid: None,
            job_uuid: None,
            progress: None,
            running_instance_count: None,
            running_core_count: None,
            execution_time: None,
            wall_time: None,
            state: None,
            previous_state: None,
            instance_count: None,
            advanced_ranges: None,
            state_transition_time: None,
            previous_state_transition_time: None,
            last_modified: None,
            creation_date: None,
            end_date: None,
            wait_for_pool_resources_synchronization: None,
        }
    }
}

impl TaskRedoInput {
    /// Configuration of the fields to change in the retry task
    pub const fn new() -> Self {
        Self {
            name: None,
            resource_buckets: None,
            advanced_resource_buckets: None,
            shortname: None,
            profile: None,
            job_uuid: None,
            result_bucket: None,
            constants: None,
            forced_constants: None,
            constraints: None,
            hardware_constraints: None,
            secrets_access_rights: None,
            tags: None,
            snapshot_whitelist: None,
            snapshot_blacklist: None,
            snapshot_bucket: None,
            snapshot_bucket_prefix: None,
            results_whitelist: None,
            results_blacklist: None,
            results_bucket: None,
            results_bucket_prefix: None,
            priority: None,
            dependencies: None,
            auto_delete_on_completion: None,
            completion_time_to_live: None,
            wait_for_pool_resources_synchronization: None,
            upload_results_on_cancellation: None,
            labels: None,
            scheduling_type: None,
            targeted_reserved_machine_key: None,
            default_resources_cache_ttl_sec: None,
            privileges: None,
            retry_settings: None,
        }
    }
}

impl TaskCloneInput {
    /// Configuration of the fields to change in the cloned task
    pub const fn new() -> Self {
        Self {
            name: None,
            instance_count: None,
            advanced_ranges: None,
            resource_buckets: None,
            advanced_resource_buckets: None,
            shortname: None,
            profile: None,
            job_uuid: None,
            result_bucket: None,
            constants: None,
            forced_constants: None,
            constraints: None,
            hardware_constraints: None,
            secrets_access_rights: None,
            tags: None,
            snapshot_whitelist: None,
            snapshot_blacklist: None,
            snapshot_bucket: None,
            snapshot_bucket_prefix: None,
            results_whitelist: None,
            results_blacklist: None,
            results_bucket: None,
            results_bucket_prefix: None,
            priority: None,
            dependencies: None,
            auto_delete_on_completion: None,
            completion_time_to_live: None,
            wait_for_pool_resources_synchronization: None,
            upload_results_on_cancellation: None,
            labels: None,
            scheduling_type: None,
            targeted_reserved_machine_key: None,
            default_resources_cache_ttl_sec: None,
            privileges: None,
            retry_settings: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_output_deserialize() {
        let example_str = r#"{
          "resourceBuckets": [
            "my-input-bucket"
          ],
          "advancedResourceBuckets": null,
          "resultBucket": "my-output-bucket",
          "completedInstances": null,
          "status": {
            "timestamp": "0001-01-01T00:00:00Z",
            "lastUpdateTimestamp": "0001-01-01T00:00:00Z",
            "downloadProgress": 100,
            "executionProgress": 0,
            "uploadProgress": 0,
            "instanceCount": 4,
            "downloadTime": "00:00:00",
            "downloadTimeSec": 0,
            "environmentTime": "00:00:00",
            "environmentTimeSec": 0,
            "executionTime": "00:00:00",
            "executionTimeSec": 0,
            "executionTimeByCpuModel": null,
            "executionTimeByMachineSpecification": null,
            "executionTimeByInstanceId": null,
            "executionTimeGhzByCpuModel": null,
            "uploadTime": "00:00:00",
            "uploadTimeSec": 0,
            "wallTime": "00:00:00",
            "wallTimeSec": 0,
            "succeededRange": null,
            "executedRange": null,
            "failedRange": null,
            "cancelledRange": null,
            "failedOnlyRange": null,
            "startedOnceRange": null,
            "runningInstancesInfo": {
              "perRunningInstanceInfo": [
                {
                  "activeForwards": null,
                  "phase": "environment",
                  "instanceId": 2467,
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
                  "specificationKey": null,
                  "cpuModel": null,
                  "coreCount": 0,
                  "executionAttemptCount": 0,
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
              "clusterPowerIndicator": 0,
              "averageMemoryUsage": 0,
              "averageNetworkInKbps": 0,
              "averageNetworkOutKbps": 0,
              "totalNetworkInKbps": 0,
              "totalNetworkOutKbps": 0,
              "runningCoreCountByCpuModel": null
            }
          },
          "snapshotInterval": 0,
          "resultsCount": 1,
          "constants": [
            {
              "key": "BLEND_FILE",
              "value": "final.blend"
            },
            {
              "key": "OTHER_KEY",
              "value": "super_value"
            }
          ],
          "secretsAccessRights": null,
          "tags": null,
          "snapshotWhitelist": "white.*",
          "snapshotBlacklist": ".*black.*",
          "uploadResultsOnCancellation": null,
          "dependencies": null,
          "autoDeleteOnCompletion": false,
          "completionTimeToLive": "00:00:00",
          "hardwareConstraints": null,
          "labels": null,
          "schedulingType": "flex",
          "privileges": null,
          "retrySettings": null,
          "uuid": "52c10b2d-0687-41e1-985e-7279f6dd543a",
          "name": "my blend",
          "shortname": "blend-task-1",
          "profile": "blender",
          "poolUuid": null,
          "jobUuid": null,
          "progress": 0,
          "runningInstanceCount": 0,
          "runningCoreCount": 0,
          "executionTime": null,
          "wallTime": null,
          "state": "Success",
          "previousState": null,
          "instanceCount": 4,
          "maxRetriesPerInstance": 0,
          "stateTransitionTime": "0001-01-01T00:00:00Z",
          "previousStateTransitionTime": "0001-01-01T00:00:00Z",
          "lastModified": "0001-01-01T00:00:00Z",
          "creationDate": "2023-12-22T14:30:58Z",
          "endDate": "0001-01-01T00:00:00Z",
          "waitForPoolResourcesSynchronization": true
        }"#;
        assert!(serde_json::from_str::<TaskOutput>(example_str).is_ok());
    }
}
