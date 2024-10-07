use serde::{Deserialize, Serialize};

/// Profile : This class gives details on a profile that can be used to launch a job with a given connection
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    /// Name of the profile.
    pub name: Option<String>,
    /// All the predefined constants in the profile, overridable by the user to configure the job.
    pub constants: Option<Vec<Constant>>,
    /// Software Licenses that are available for the profile.
    pub licenses: Option<Vec<License>>,
}

/// Constant : This class represents a constant in a profile description.
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Constant {
    /// Name of the constant.
    pub name: Option<String>,
    /// Value of the constant.
    pub value: Option<String>,
    /// A description ofwhat the constant does.
    pub description: Option<String>,
}

/// License : This class represents information on software license available for a profile
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct License {
    /// Name of the software
    pub name: Option<String>,
    /// Maximum number of concurrent instances of a license that can be used.
    pub max_instances: Option<u64>,
    /// Maximum number of cores availbale for the license.
    pub max_cores: Option<u64>,
}

impl Profile {
    /// This class gives details on a profile that can be used to launch a job with a given connection
    pub const fn new() -> Self {
        Self {
            name: None,
            constants: None,
            licenses: None,
        }
    }
}
