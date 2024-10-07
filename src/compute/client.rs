use crate::compute::models;
use crate::compute::models::UserInfo;
use crate::compute::models::Version;
use crate::compute::ComputeError;
use reqwest::header;
use reqwest::{Response, StatusCode};
use serde::Serialize;
use std::collections::HashMap;

const APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

pub struct ComputeClient {
    /// HTTP client
    client: reqwest::Client,
    /// Url of the compute API
    compute_url: String,
    /// Compute API version
    version: String,
}

impl ComputeClient {
    pub fn new(compute_url: String, version: String, api_key: &str) -> Result<Self, ComputeError> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );
        let mut auth_value =
            header::HeaderValue::from_str(api_key).map_err(|_| ComputeError::Unauthorized)?;
        auth_value.set_sensitive(true);
        headers.insert(header::AUTHORIZATION, auth_value);
        let client = reqwest::Client::builder()
            .user_agent(APP_USER_AGENT)
            .default_headers(headers)
            .build()
            .map_err(|_| ComputeError::Generic)?;

        Ok(Self {
            client,
            compute_url,
            version,
        })
    }

    fn check_status(response: Response) -> Result<Response, ComputeError> {
        match response.status() {
            StatusCode::OK => Ok(response),
            StatusCode::UNAUTHORIZED => Err(ComputeError::Unauthorized),
            StatusCode::FORBIDDEN => Err(ComputeError::Forbidden),
            StatusCode::NOT_FOUND => Err(ComputeError::NotFound),
            _ => Err(ComputeError::Generic),
        }
    }

    /// Send a GET request
    ///
    /// # Arguments
    /// * `self` - The [`ComputeClient`]
    /// * `query` - An optional [`Vec`] of [`&str`] pairs representing the query
    async fn get_request(
        &self,
        route: &str,
        query: Option<Vec<(&str, &str)>>,
    ) -> Result<reqwest::Response, ComputeError> {
        let mut request = self
            .client
            .get(format!("{}/{}/{}", self.compute_url, self.version, route));

        if let Some(query) = query {
            request = request.query(&query);
        }
        let response = request.send().await;
        match response {
            Ok(response) => Self::check_status(response),
            Err(e) => {
                error!("API Error: {}", e);
                Err(ComputeError::Generic)
            }
        }
    }

    async fn post_request<T>(
        &self,
        route: &str,
        body: Option<T>,
        query: Option<Vec<(&str, &str)>>,
    ) -> Result<reqwest::Response, ComputeError>
    where
        T: Serialize + Send,
    {
        let mut request = self
            .client
            .post(format!("{}/{}/{}", self.compute_url, self.version, route));

        if let Some(query) = query {
            request = request.query(&query);
        }

        if let Some(body) = body {
            request = request.json(&body);
        }

        let response = request.send().await;
        match response {
            Ok(response) => Self::check_status(response),
            Err(e) => {
                error!("API Error: {}", e);
                Err(ComputeError::Generic)
            }
        }
    }

    async fn put_request<T>(
        &self,
        route: &str,
        body: Option<T>,
        query: Option<Vec<(&str, &str)>>,
    ) -> Result<reqwest::Response, ComputeError>
    where
        T: Serialize + Send,
    {
        let mut request = self
            .client
            .put(format!("{}/{}/{}", self.compute_url, self.version, route));

        if let Some(query) = query {
            request = request.query(&query);
        }

        if let Some(body) = body {
            request = request.json(&body);
        }

        let response = request.send().await;
        match response {
            Ok(response) => Self::check_status(response),
            Err(e) => {
                error!("API Error: {}", e);
                Err(ComputeError::Generic)
            }
        }
    }

    async fn patch_request(&self, route: &str) -> Result<reqwest::Response, ComputeError> {
        let response = self
            .client
            .patch(format!("{}/{}/{}", self.compute_url, self.version, route))
            .send()
            .await;
        match response {
            Ok(response) => Self::check_status(response),
            Err(e) => {
                error!("API Error: {}", e);
                Err(ComputeError::Generic)
            }
        }
    }

    async fn delete_request(&self, route: &str) -> Result<reqwest::Response, ComputeError> {
        let response = self
            .client
            .delete(format!("{}/{}/{}", self.compute_url, self.version, route))
            .send()
            .await;
        match response {
            Ok(response) => Self::check_status(response),
            Err(e) => {
                error!("API Error: {}", e);
                Err(ComputeError::Generic)
            }
        }
    }

    /// Get info on the current user
    ///
    /// # Arguments
    /// * `self` - The [`ComputeClient`]
    pub async fn get_user_info(&self) -> Result<UserInfo, ComputeError> {
        let resp = self.get_request("info", None).await?;
        debug!("reponse status {}", resp.status());
        let info = resp.json::<UserInfo>().await;
        match info {
            Ok(info) => Ok(info),
            Err(e) => {
                error!("deserialize error {}", e);
                Err(ComputeError::Generic)
            }
        }
    }

    /// Get list of API versions
    ///
    /// # Arguments
    /// * `self` - The [`ComputeClient`]
    pub async fn get_versions(&self) -> Result<Vec<Version>, ComputeError> {
        let resp = self.get_request("versions", None).await?;
        let versions = resp.json::<Vec<Version>>().await;
        match versions {
            Ok(versions) => Ok(versions),
            Err(e) => {
                error!("deserialize error {}", e);
                Err(ComputeError::Generic)
            }
        }
    }

    /// Get API status (and verify user auth)
    ///
    /// # Arguments
    /// * `self` - The [`ComputeClient`]
    pub async fn get_status(&self) -> Result<(), ComputeError> {
        let _ = self.get_request("", None).await?;
        Ok(())
    }

    /// Get all user tasks
    ///
    /// # Arguments
    /// * `self` - The [`ComputeClient`]
    /// * `tags` - A slice of [`&str`] for filtering tasks by tags
    pub async fn get_tasks(
        &self,
        tags: Option<&[&str]>,
    ) -> Result<Vec<models::TaskOutput>, ComputeError> {
        let mut query: Option<Vec<(&str, &str)>> = None;
        if let Some(tags) = tags {
            query = Some(
                tags.iter()
                    .map(|e| ("tag", *e))
                    .collect::<Vec<(&str, &str)>>(),
            );
        }
        let resp = self.get_request("/tasks", query).await?;
        let tasks = resp.json::<Vec<models::TaskOutput>>().await;
        match tasks {
            Ok(tasks) => Ok(tasks),
            Err(e) => {
                error!("deserialize error {}", e);
                Err(ComputeError::Generic)
            }
        }
    }

    /// Get all user tasks' summaries
    ///
    /// # Arguments
    /// * `self` - The [`ComputeClient`]
    /// * `tags` - A slice of [`&str`] for filtering tasks by tags
    pub async fn get_tasks_summaries(
        &self,
        tags: Option<&[&str]>,
    ) -> Result<Vec<models::TaskSummaryOutput>, ComputeError> {
        let mut query: Option<Vec<(&str, &str)>> = None;
        if let Some(tags) = tags {
            query = Some(
                tags.iter()
                    .map(|e| ("tag", *e))
                    .collect::<Vec<(&str, &str)>>(),
            );
        }
        let resp = self.get_request("/tasks/summaries", query).await?;
        let tasks = resp.json::<Vec<models::TaskSummaryOutput>>().await;
        match tasks {
            Ok(tasks) => Ok(tasks),
            Err(e) => {
                error!("deserialize error {}", e);
                Err(ComputeError::Generic)
            }
        }
    }

    /// Get information of the specified task
    ///
    /// * `self` - The [`ComputeClient`]
    /// * `uuid` - Uuid of the task to resume
    pub async fn get_task_info(
        &self,
        uuid: uuid::Uuid,
    ) -> Result<models::TaskOutput, ComputeError> {
        let resp = self.get_request(&format!("tasks/{uuid}"), None).await?;
        let task = resp.json::<models::TaskOutput>().await;
        match task {
            Ok(task) => Ok(task),
            Err(e) => {
                error!("deserialize error {}", e);
                Err(ComputeError::Generic)
            }
        }
    }

    /// Get hardware constraints
    ///
    /// # Arguments
    /// * `self` - The [`ComputeClient`]
    pub async fn get_hardware_constraints(
        &self,
    ) -> Result<models::HardwareConstraintResponse, ComputeError> {
        let resp = self.get_request("hardware-constraints", None).await?;
        let hwc = resp.json::<models::HardwareConstraintResponse>().await;
        match hwc {
            Ok(hwc) => Ok(hwc),
            Err(e) => {
                error!("deserialize error {}", e);
                Err(ComputeError::Generic)
            }
        }
    }

    /// Get profiles
    ///
    /// # Arguments
    /// * `self` - The [`ComputeClient`]
    pub async fn get_profiles(&self) -> Result<Vec<String>, ComputeError> {
        let resp = self.get_request("profiles", None).await?;
        let profiles = resp.json::<Vec<String>>().await;
        match profiles {
            Ok(profiles) => Ok(profiles),
            Err(e) => {
                error!("deserialize error {}", e);
                Err(ComputeError::Generic)
            }
        }
    }

    /// Get profile details
    ///
    /// # Arguments
    /// * `self` - The [`ComputeClient`]
    pub async fn get_profile_details(
        &self,
        profile_name: &str,
    ) -> Result<models::Profile, ComputeError> {
        let path = format!("profiles/{profile_name}");
        let resp = self.get_request(&path, None).await?;
        let profile = resp.json::<models::Profile>().await;
        match profile {
            Ok(profile) => Ok(profile),
            Err(e) => {
                error!("deserialize error {}", e);
                Err(ComputeError::Generic)
            }
        }
    }

    /// Get public settings
    ///
    /// # Arguments
    /// * `self` - The [`ComputeClient`]
    pub async fn get_settings(&self) -> Result<HashMap<String, String>, ComputeError> {
        let resp = self.get_request("settings", None).await?;
        let settings = resp.json::<HashMap<String, String>>().await;
        match settings {
            Ok(settings) => Ok(settings),
            Err(e) => {
                error!("deserialize error {}", e);
                Err(ComputeError::Generic)
            }
        }
    }

    /// Start a new task
    ///
    /// # Arguments
    /// * `self` - The [`ComputeClient`]
    /// * `task` - the task to create as [`TaskCreationInput`]
    pub async fn post_task(
        &self,
        task: models::TaskCreationInput,
    ) -> Result<models::Id, ComputeError> {
        let resp = self.post_request("tasks", Some(task), None).await?;
        let res = resp.json::<models::Id>().await;
        match res {
            Ok(uuid) => Ok(uuid),
            Err(e) => {
                error!("deserialize error {}", e);
                Err(ComputeError::Generic)
            }
        }
    }

    /// Retry a task
    ///
    /// # Arguments
    /// * `self` - the [`ComputeClient`]
    /// * `uuid` - Uuid of the task to retry
    /// * `task` - the [`TaskRedoInput`]
    pub async fn post_retry_task(
        &self,
        uuid: uuid::Uuid,
        task: models::TaskRedoInput,
    ) -> Result<models::Id, ComputeError> {
        let resp = self
            .post_request(&format!("tasks/{uuid}/retry"), Some(task), None)
            .await?;
        let res = resp.json::<models::Id>().await;
        match res {
            Ok(uuid) => Ok(uuid),
            Err(e) => {
                error!("deserialize error {}", e);
                Err(ComputeError::Generic)
            }
        }
    }

    /// Recover a task
    ///
    /// # Arguments
    /// * `self` - the [`ComputeClient`]
    /// * `uuid` - Uuid of the task to recover
    /// * `task` - the [`TaskRedoInput`]
    pub async fn post_recover_task(
        &self,
        uuid: uuid::Uuid,
        task: models::TaskRedoInput,
    ) -> Result<models::Id, ComputeError> {
        let resp = self
            .post_request(&format!("tasks/{uuid}/recover"), Some(task), None)
            .await?;
        let res = resp.json::<models::Id>().await;
        match res {
            Ok(uuid) => Ok(uuid),
            Err(e) => {
                error!("deserialize error {}", e);
                Err(ComputeError::Generic)
            }
        }
    }

    /// Resume a task
    ///
    /// # Arguments
    /// * `self` - the [`ComputeClient`]
    /// * `uuid` - Uuid of the task to resume
    /// * `task` - the [`TaskRedoInput`]
    pub async fn post_resume_task(
        &self,
        uuid: uuid::Uuid,
        task: models::TaskRedoInput,
    ) -> Result<models::Id, ComputeError> {
        let resp = self
            .post_request(&format!("tasks/{uuid}/resume"), Some(task), None)
            .await?;
        let res = resp.json::<models::Id>().await;
        match res {
            Ok(uuid) => Ok(uuid),
            Err(e) => {
                error!("deserialize error {}", e);
                Err(ComputeError::Generic)
            }
        }
    }

    /// Update a running task
    ///
    /// # Arguments
    /// * `self` - the [`ComputeClient`]
    /// * `uuid` - Uuid of the task to update
    /// * `task` - the [`TaskUpdateInput`]
    pub async fn put_update_task(
        &self,
        uuid: uuid::Uuid,
        task: models::TaskUpdateInput,
    ) -> Result<(), ComputeError> {
        let resp = self
            .put_request(&format!("tasks/{uuid}"), Some(task), None)
            .await;
        match resp {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("Failed to update task {:?}", e);
                Err(e)
            }
        }
    }

    /// Trigger resource update of running task on on compute nodes
    ///
    /// Compute nodes will see new files uploaded to the bucket since the last
    /// resources synchronization (either at node provisioning,
    /// or from an earlier call to this method) and
    /// get an updated version of files that were modified in the bucket.
    /// NOTE: Files that were deleted from the bucket since the last
    /// synchronization WILL NOT be deleted from compute nodes.
    ///
    /// # Arguments
    /// * `self` - the [`ComputeClient`]
    /// * `uuid` - Uuid of the task
    pub async fn patch_update_task_resources(&self, uuid: uuid::Uuid) -> Result<(), ComputeError> {
        let resp = self.patch_request(&format!("tasks/{uuid}")).await;
        match resp {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("Failed to update task resources {:?}", e);
                Err(e)
            }
        }
    }

    /// Delete a task (abort if needed)
    ///
    /// # Arguments
    /// * `self` - the [`ComputeClient`]
    /// * `uuid` - Uuid of the task to delete
    pub async fn delete_task(&self, uuid: uuid::Uuid) -> Result<(), ComputeError> {
        let resp = self.delete_request(&format!("tasks/{uuid}")).await;
        match resp {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("Failed to delete task {:?}", e);
                Err(e)
            }
        }
    }

    /// Abort a task
    ///
    /// # Arguments
    /// * `self` - the [`ComputeClient`]
    /// * `uuid` - Uuid of the task to abort
    pub async fn post_abort_task(&self, uuid: uuid::Uuid) -> Result<(), ComputeError> {
        let resp = self
            .post_request::<()>(&format!("tasks/{uuid}/abort"), Some(()), None)
            .await;
        match resp {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("Failed to abort task {:?}", e);
                Err(e)
            }
        }
    }

    /// Get task standard output (can be rotated)
    ///
    /// # Arguments
    /// * `self` - the [`ComputeClient`]
    /// * `uuid` - Uuid of the task
    pub async fn get_task_stdout(&self, uuid: uuid::Uuid) -> Result<String, ComputeError> {
        let resp = self
            .get_request(&format!("tasks/{uuid}/stdout"), None)
            .await?;
        resp.text().await.map_err(|_| ComputeError::Generic)
    }

    /// Get task standard error (can be rotated)
    ///
    /// # Arguments
    /// * `self` - the [`ComputeClient`]
    /// * `uuid` - Uuid of the task
    pub async fn get_task_stderr(&self, uuid: uuid::Uuid) -> Result<String, ComputeError> {
        let resp = self
            .get_request(&format!("tasks/{uuid}/stderr"), None)
            .await?;
        resp.text().await.map_err(|_| ComputeError::Generic)
    }

    /// Get task last standard output (can be rotated)
    ///
    /// # Arguments
    /// * `self` - the [`ComputeClient`]
    /// * `uuid` - Uuid of the task
    pub async fn post_task_last_stdout(&self, uuid: uuid::Uuid) -> Result<String, ComputeError> {
        let resp = self
            .post_request::<()>(&format!("tasks/{uuid}/stdout"), None, None)
            .await?;
        resp.text().await.map_err(|_| ComputeError::Generic)
    }

    /// Get task last standard error (can be rotated)
    ///
    /// # Arguments
    /// * `self` - the [`ComputeClient`]
    /// * `uuid` - Uuid of the task
    pub async fn post_task_last_stderr(&self, uuid: uuid::Uuid) -> Result<String, ComputeError> {
        let resp = self
            .post_request::<()>(&format!("tasks/{uuid}/stderr"), None, None)
            .await?;
        resp.text().await.map_err(|_| ComputeError::Generic)
    }

    /// Get instance standard output (can be rotated)
    ///
    /// # Arguments
    /// * `self` - the [`ComputeClient`]
    /// * `uuid` - Uuid of the task
    /// * `instance_id` - Id of the instance
    pub async fn get_instance_stdout(
        &self,
        uuid: uuid::Uuid,
        instance_id: u32,
    ) -> Result<String, ComputeError> {
        let resp = self
            .get_request(&format!("tasks/{uuid}/stdout/{instance_id}"), None)
            .await?;
        resp.text().await.map_err(|_| ComputeError::Generic)
    }

    /// Get instance standard error (can be rotated)
    ///
    /// # Arguments
    /// * `self` - the [`ComputeClient`]
    /// * `uuid` - Uuid of the task
    pub async fn get_instance_stderr(
        &self,
        uuid: uuid::Uuid,
        instance_id: u32,
    ) -> Result<String, ComputeError> {
        let resp = self
            .get_request(&format!("tasks/{uuid}/stderr/{instance_id}"), None)
            .await?;
        resp.text().await.map_err(|_| ComputeError::Generic)
    }

    /// Get instance last standard output (can be rotated)
    ///
    /// # Arguments
    /// * `self` - the [`ComputeClient`]
    /// * `uuid` - Uuid of the task
    pub async fn post_instance_last_stdout(
        &self,
        uuid: uuid::Uuid,
        instance_id: u32,
    ) -> Result<String, ComputeError> {
        let resp = self
            .post_request::<()>(&format!("tasks/{uuid}/stdout/{instance_id}"), None, None)
            .await?;
        resp.text().await.map_err(|_| ComputeError::Generic)
    }

    /// Get instance last standard error (can be rotated)
    ///
    /// # Arguments
    /// * `self` - the [`ComputeClient`]
    /// * `uuid` - Uuid of the task
    pub async fn post_instance_last_stderr(
        &self,
        uuid: uuid::Uuid,
        instance_id: u32,
    ) -> Result<String, ComputeError> {
        let resp = self
            .post_request::<()>(&format!("tasks/{uuid}/stderr/{instance_id}"), None, None)
            .await?;
        resp.text().await.map_err(|_| ComputeError::Generic)
    }
}
