use toml::Value;

use crate::config::ConfigError::MissingConfig;
use crate::config::{ConfigComponent, ConfigError};

#[derive(Debug, Clone)]
pub struct General {
    pub port: u16,
    pub log_path: String,
    pub refresh_rate: u64,
}

impl ConfigComponent for General {
    type ConfigType = General;

    fn parse(config_file: Value) -> Result<Self::ConfigType, ConfigError> {
        let general = match config_file.get("General") {
            None => return Err(MissingConfig("General".to_string())),
            Some(general) => general,
        };

        let port = match general.get("port") {
            None => 3000u16,
            Some(port) => {
                if let Some(port) = port.as_integer() {
                    port as u16
                } else {
                    3000u16
                }
            }
        };

        let log_path = match general.get("log_path") {
            None => "/var/log/hiko.log".to_string(),
            Some(log_path) => log_path.as_str().to_owned().unwrap().to_string(),
        };

        let refresh_rate = match general.get("refresh_rate") {
            None => 30,
            Some(refresh_rate) => {
                if let Some(rate) = refresh_rate.as_integer() {
                    rate as u64
                } else {
                    30
                }
            }
        };

        Ok(General {
            port,
            log_path,
            refresh_rate,
        })
    }
}
