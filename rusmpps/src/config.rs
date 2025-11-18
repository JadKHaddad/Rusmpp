use std::{
    net::SocketAddr,
    path::{Path, PathBuf},
    time::Duration,
};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub socket_addr: SocketAddr,
    #[serde(with = "humantime_serde")]
    #[serde(default)]
    pub enquire_link_interval: Option<Duration>,
    #[serde(with = "humantime_serde")]
    pub enquire_link_response_timeout: Duration,
    #[serde(with = "humantime_serde")]
    pub session_timeout: Duration,
    #[serde(with = "humantime_serde")]
    #[serde(default)]
    pub bind_delay: Option<Duration>,
    #[serde(with = "humantime_serde")]
    #[serde(default)]
    pub response_delay: Option<Duration>,
    #[serde(with = "humantime_serde")]
    #[serde(default)]
    pub enquire_link_response_delay: Option<Duration>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            enquire_link_interval: Some(Duration::from_secs(10)),
            enquire_link_response_timeout: Duration::from_secs(3),
            enquire_link_response_delay: Some(Duration::from_millis(100)),
            session_timeout: Duration::from_secs(3),
            bind_delay: Some(Duration::from_millis(100)),
            response_delay: Some(Duration::from_millis(100)),
            socket_addr: "127.0.0.1:2775"
                .parse()
                .expect("Failed to parse socket address"),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum LoadConfigErrorKind {
    #[error("Failed to read config file: {0}")]
    Read(#[from] std::io::Error),
    #[error("Failed to parse yaml: {0}")]
    Parse(#[from] serde_yaml::Error),
}

#[derive(Debug, thiserror::Error)]
#[error("Failed to load configuration from `{path}`: {kind}")]
pub struct LoadConfigError {
    path: PathBuf,
    kind: LoadConfigErrorKind,
}

impl Config {
    fn from_yaml(yaml: &[u8]) -> Result<Self, serde_yaml::Error> {
        serde_yaml::from_slice(yaml)
    }

    pub fn from_yaml_file(path: impl AsRef<Path>) -> Result<Self, LoadConfigError> {
        let path = path.as_ref();

        let yaml = std::fs::read(path).map_err(|err| LoadConfigError {
            path: path.into(),
            kind: LoadConfigErrorKind::Read(err),
        })?;

        Self::from_yaml(&yaml).map_err(|err| LoadConfigError {
            path: path.into(),
            kind: LoadConfigErrorKind::Parse(err),
        })
    }
}
