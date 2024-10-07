use ini::Ini;
use std::path::Path;

pub struct Config {
    pub api_url: String,
    pub api_key: String,
    pub version: String,
    pub storage_url: Option<String>,
}

impl Config {
    /// Create a new client config
    pub fn new(
        api_url: &str,
        api_key: &str,
        version: Option<&str>,
        storage_url: Option<String>,
    ) -> Self {
        Self {
            api_url: api_url.to_owned(),
            api_key: api_key.to_owned(),
            version: version.unwrap_or("v1").to_owned(),
            storage_url,
        }
    }

    /// Load a client config from ini file.
    /// You should be able to bring the same config as the one used
    /// for the Python SDK
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let conf = Ini::load_from_file(path);
        match conf {
            Ok(conf) => {
                let mut api_url: Option<String> = None;
                let mut api_key: Option<String> = None;
                let mut storage_url: Option<String> = None;
                let mut version: Option<String> = None;
                if let Some(section) = conf.section(Some("cluster")) {
                    if let Some(url) = section.get("url") {
                        api_url = Some(url.to_owned());
                    }
                    if let Some(ver) = section.get("version") {
                        version = Some(ver.to_owned());
                    }
                }
                if let Some(section) = conf.section(Some("client")) {
                    if let Some(token) = section.get("token") {
                        api_key = Some(token.to_owned());
                    }
                }
                if let Some(section) = conf.section(Some("storage")) {
                    if let Some(url) = section.get("url") {
                        storage_url = Some(url.to_owned());
                    }
                }
                if let (Some(api_url), Some(api_key)) = (api_url, api_key) {
                    Ok(Self {
                        api_url,
                        api_key,
                        version: version.unwrap_or_else(|| String::from("v1")),
                        storage_url,
                    })
                } else {
                    Err(Error::InvalidConfig)
                }
            }
            Err(ini::Error::Io(_)) => Err(Error::FileNotFound),
            Err(ini::Error::Parse(_)) => Err(Error::InvalidConfig),
        }
    }
}

/// Possible error when loading config from file
#[derive(Debug)]
pub enum Error {
    FileNotFound,
    InvalidConfig,
}
