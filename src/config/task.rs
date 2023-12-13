use crate::config::ConfigError::{InvalidFile, MissingField};
use crate::config::{ConfigComponent, ConfigError};
use toml::Value;

#[derive(Debug, PartialEq)]
pub struct Task {
    pub timeout: u64,
}

impl ConfigComponent for Task {
    type ConfigType = Option<Task>;

    fn parse(config_file: Value) -> Result<Self::ConfigType, ConfigError> {
        Ok(match config_file.get("Task") {
            Some(task) => {
                let timeout = match task.get("timeout") {
                    Some(timeout) => {
                        if let Ok(timeout) = timeout.to_string().parse::<u64>() {
                            timeout
                        } else {
                            return Err(InvalidFile("Task::timeout: u64".to_string()));
                        }
                    }
                    None => {
                        return Err(MissingField("Task::timeout".to_string()));
                    }
                };
                Some(Task { timeout })
            }
            None => None,
        })
    }
}
