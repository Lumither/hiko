use toml::Value;

use crate::config::ConfigError::MissingConfig;
use crate::config::{ConfigComponent, ConfigError};

#[derive(Debug, Copy, Clone)]
pub struct General {
    pub port: u16,
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

        Ok(General { port })
    }
}
