use crate::config::ConfigError::{MissingConfig, MissingField};
use crate::config::{ConfigComponent, ConfigError};
use toml::Value;

#[derive(Debug)]
pub struct Database {
    pub url: String,
    pub user: String,
    pub password: String,
}

impl ConfigComponent for Database {
    type ConfigType = Database;

    fn parse(config_file: Value) -> Result<Self::ConfigType, ConfigError> {
        let database = match config_file.get("Database") {
            None => return Err(MissingConfig("Database".to_string())),
            Some(database) => database,
        };

        let url = match database.get("url") {
            None => return Err(MissingField("Database::url".to_string())),
            Some(url) => url.as_str().to_owned().unwrap().to_string(),
        };
        let user = match database.get("user") {
            None => return Err(MissingField("Database::user".to_string())),
            Some(user) => user.as_str().to_owned().unwrap().to_string(),
        };
        let password = match database.get("password") {
            None => return Err(MissingField("Database::password".to_string())),
            Some(password) => password.as_str().to_owned().unwrap().to_string(),
        };
        Ok(Database {
            url,
            user,
            password,
        })
    }
}
