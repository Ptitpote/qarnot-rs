use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourcesBucket {
    pub bucket_name: Option<String>,
    pub filtering: ResourcesFiltering,
    pub resources_transformation: ResourcesTransformation,
    #[serde(rename = "cacheTTLSec")]
    pub cache_ttl_sec: Option<u32>,
}

#[derive(Clone, Deserialize, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourcesFiltering {
    pub prefix_filtering: PrefixFiltering,
}

#[derive(Clone, Deserialize, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PrefixFiltering {
    pub prefix: Option<String>,
}

#[derive(Clone, Deserialize, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourcesTransformation {
    pub strip_prefix: StripPrefix,
}

#[derive(Clone, Deserialize, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StripPrefix {
    pub prefix: Option<String>,
}
