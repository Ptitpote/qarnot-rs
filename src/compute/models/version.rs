use serde::{Deserialize, Serialize};

// TODO end_of_life conversion to actual Date type
/// Represents a version of the API
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Version {
    /// Version name (vX.Y)
    version: String,
    /// End of life of the version
    end_of_life: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_deserialize() {
        let example_str = r#"
            {
                "version": "v0.1",
                "endOfLife": "2021-04-06"
            }"#;
        assert!(serde_json::from_str::<Version>(example_str).is_ok());
        let example_str = r#"
            {
                "version": "v0.2",
                "endOfLife": null
            }
            "#;
        assert!(serde_json::from_str::<Version>(example_str).is_ok());
    }
}
