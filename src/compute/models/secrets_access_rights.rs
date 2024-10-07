use serde::{Deserialize, Serialize};

/// SecretsAccessRights : Describe secrets the task or pool will have access to  when running.
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SecretsAccessRights {
    /// Describe secrets by using their exact path.
    pub by_secret: Option<Vec<ExactSecretAccessRight>>,
    /// Describe secrets by using a prefix. All secrets starting  with any of the given prefix will be available to the task.
    pub by_prefix: Option<Vec<PrefixSecretAccessRight>>,
}

/// ExactSecretAccessRight : Give access to a single secret, using its  full path.
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct ExactSecretAccessRight {
    /// The secret to give access to.
    pub key: Option<String>,
}

/// PrefixFilter : Filtering by prefix
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct PrefixSecretAccessRight {
    /// Prefix used for filtering
    pub prefix: Option<String>,
}
