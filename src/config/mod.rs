use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::fs;

use toml::Value;

use crate::config::database::Database;
use crate::config::general::General;
use crate::config::mail::Mail;
use crate::config::task::Task;
use crate::config::ConfigError::{FileNotFound, InvalidFile};

mod database;
pub(crate) mod mail;
mod task;

mod general;
#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct Config {
    pub general: General,
    pub database: Database,
    pub task: Option<Task>,
    pub mail: Option<Mail>,
}

pub enum ConfigError {
    MissingConfig(String),
    MissingField(String),
    InvalidFile(String),
    InvalidField(String),
    FileNotFound(String),
}

impl Debug for ConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::MissingConfig(desc) => write!(f, "Missing configuration: {}", desc),
            ConfigError::MissingField(desc) => write!(f, "Missing field: {}", desc),
            ConfigError::InvalidFile(desc) => write!(f, "Invalid file: {}", desc),
            ConfigError::InvalidField(desc) => write!(f, "Invalid field: {}", desc),
            ConfigError::FileNotFound(desc) => write!(f, "File not found: {}", desc),
        }
    }
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Configuration Error: {:?}", &self) // Debug::fmt
    }
}

impl Error for ConfigError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

pub trait ConfigComponent {
    type ConfigType;
    fn parse(config_file: Value) -> Result<Self::ConfigType, ConfigError>;
}

impl Config {
    pub fn from(config_path: &str) -> Result<Config, ConfigError> {
        let config_file = read_toml(config_path)?;

        // General
        let general = General::parse(config_file.clone())?;

        // Database
        let database = Database::parse(config_file.clone())?;

        // Task
        let task = Task::parse(config_file.clone())?;

        // Mail
        let mail = Mail::parse(config_file.clone())?;

        Ok(Config {
            general,
            database,
            task,
            mail,
        })
    }
}

pub fn read_toml(file_path: &str) -> Result<Value, ConfigError> {
    if let Ok(file_contents) = fs::read_to_string(file_path) {
        if let Ok(toml_value) = toml::from_str(&file_contents) {
            Ok(toml_value)
        } else {
            Err(InvalidFile(file_path.to_string()))
        }
    } else {
        // todo: create template conf file
        Err(FileNotFound(file_path.to_string()))
    }
}

#[cfg(test)]
#[test]
fn read_config() {
    let cfg = Config::from("./Config.toml").unwrap();
    print!("{:?}", cfg);
}
