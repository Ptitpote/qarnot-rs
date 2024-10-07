use serde::{Deserialize, Serialize};

/// SchedulingClass : Type of scheduling used when dispatching the tasks
/// Type of scheduling used when dispatching the tasks
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SchedulingClass {
    Flex,
    OnDemand,
    Reserved,
}

impl std::fmt::Display for SchedulingClass {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Flex => write!(f, "flex"),
            Self::OnDemand => write!(f, "onDemand"),
            Self::Reserved => write!(f, "reserved"),
        }
    }
}

impl Default for SchedulingClass {
    fn default() -> Self {
        Self::Flex
    }
}
