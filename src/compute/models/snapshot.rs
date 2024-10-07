use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PeriodicSnapshot {
    /// Time interval (in sec) between each periodic snapshot.
    pub interval: Option<i32>,
    /// Whitelist filter.
    pub whitelist: Option<String>,
    /// Blacklist filter.
    pub blacklist: Option<String>,
    /// Bucket name.
    pub bucket: Option<String>,
    /// Bucket prefix.
    pub bucket_prefix: Option<String>,
}

#[derive(Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UniqueSnapshot {
    /// Whitelist filter.
    pub whitelist: Option<String>,
    /// Blacklist filter.
    pub blacklist: Option<String>,
    /// Bucket name.
    pub bucket: Option<String>,
    /// Bucket prefix.
    pub bucket_prefix: Option<String>,
}

impl PeriodicSnapshot {
    pub const fn new() -> Self {
        Self {
            interval: None,
            whitelist: None,
            blacklist: None,
            bucket: None,
            bucket_prefix: None,
        }
    }
}

impl UniqueSnapshot {
    pub const fn new() -> Self {
        Self {
            whitelist: None,
            blacklist: None,
            bucket: None,
            bucket_prefix: None,
        }
    }
}
