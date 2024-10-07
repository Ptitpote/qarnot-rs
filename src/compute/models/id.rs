use serde::{Deserialize, Serialize};

/// Id: A uuid of a pool/task/job...
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Id {
    /// The uuid
    pub uuid: Option<uuid::Uuid>,
}
