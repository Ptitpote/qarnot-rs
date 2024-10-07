use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    pub email: Option<String>,
    pub max_bucket: u32,
    pub max_task: u32,
    pub task_count: u32,
    pub max_job: u32,
    pub job_count: u32,
    pub max_pool: u32,
    pub pool_count: u32,
    pub max_running_task: u32,
    pub max_running_pool: u32,
    pub running_task_count: u32,
    pub running_pool_count: u32,
    pub running_instance_count: u32,
    pub running_core_count: u32,
    pub max_flex_instances: u32,
    pub max_flex_cores: u32,
    pub max_on_demand_instances: u32,
    pub max_on_demand_cores: u32,
    pub reserved_quotas: Option<Vec<ReservedQuotas>>,
    pub quota_bytes: u64,
    pub quota_bytes_bucket: u64,
    pub used_quota_bytes_bucket: u64,
    pub used_quota_bytes: u64,
    pub default_scheduling: Option<String>,
    pub default_reserved_specification_key: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReservedQuotas {
    pub machine_key: String,
    pub max_instances: u32,
    pub max_cores: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_deserialize() {
        let example_str = r#"{
            "email": "admin@mail.com",
            "maxBucket": 10000,
            "maxTask": 10000,
            "taskCount": 0,
            "maxJob": 0,
            "jobCount": 0,
            "maxPool": 10000,
            "poolCount": 0,
            "maxRunningTask": 10000,
            "maxRunningPool": 10000,
            "runningTaskCount": 0,
            "runningPoolCount": 0,
            "runningInstanceCount": 0,
            "runningCoreCount": 0,
            "maxInstances": 10000,
            "maxCores": 10000,
            "maxFlexInstances": 10000,
            "maxFlexCores": 10000,
            "maxOnDemandInstances": 0,
            "maxOnDemandCores": 0,
            "reservedQuotas": [],
            "quotaBytes": 0,
            "quotaBytesBucket": 1000000000,
            "usedQuotaBytesBucket": 0,
            "usedQuotaBytes": 0,
            "defaultScheduling": "Flex",
            "defaultReservedSpecificationKey": null}"#;
        assert!(serde_json::from_str::<UserInfo>(example_str).is_ok());
    }
}
