use serde::{Deserialize, Serialize};

/// ForcedConstant : Describe a constant to be overriden when running the task.  <br />This is meant to be used for development only and require  specific permissions.
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForcedConstant {
    /// The name of the constant to override.
    pub constant_name: Option<String>,
    /// The new value for the constant. The value is  unchanged if the field is not present or null.
    pub forced_value: Option<String>,
    /// Change whether the constant should be exported  to the environment or not. The behaviour is unchanged  if the field is not present or null.
    pub force_export_in_environment: Option<bool>,
    pub access: Option<ForcedConstantAccess>,
}

/// ForcedConstantAccess : Possible values for the Access property of a  ForcedConstant object.
/// Possible values for the Access property of a  ForcedConstant object.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ForcedConstantAccess {
    ReadOnly,
    ReadWrite,
}

impl std::fmt::Display for ForcedConstantAccess {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ReadOnly => write!(f, "readOnly"),
            Self::ReadWrite => write!(f, "readWrite"),
        }
    }
}

impl Default for ForcedConstantAccess {
    fn default() -> Self {
        Self::ReadOnly
    }
}

impl ForcedConstant {
    /// Describe a constant to be overriden when running the task.  <br />This is meant to be used for development only and require  specific permissions.
    pub const fn new() -> Self {
        Self {
            constant_name: None,
            forced_value: None,
            force_export_in_environment: None,
            access: None,
        }
    }
}
