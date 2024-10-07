use serde::Serialize;

/// DependencyInput : Initial task dependency
#[derive(Clone, Default, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DependencyInput {
    /// List of tasks' Uuid the task depends on.  The task will start only if these tasks are completed
    pub depends_on: Option<Vec<uuid::Uuid>>,
}

impl DependencyInput {
    /// Initial task dependency
    pub const fn new() -> Self {
        Self { depends_on: None }
    }
}
