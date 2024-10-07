use serde::{Deserialize, Serialize};

/// Privileges : List of privileges that can be granted for task execution
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Privileges {
    /// Allow the api and storage credentials to be exported into the environment through constants  Default value is false.
    pub export_api_and_storage_credentials_in_environment: Option<bool>,
}
