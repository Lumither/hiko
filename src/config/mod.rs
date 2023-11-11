use std::fs;

use toml::Value;

mod test_config;

#[derive(Debug)]
pub struct Config {
    pub db_path: String,
    pub task: Option<Task>,
    pub mail: Option<Mail>,
}

#[derive(Debug)]
pub struct Mail {
    pub address: String,
    pub password: String,
}

#[derive(Debug, PartialEq)]
pub struct Task {
    pub timeout: u64,
}

impl Config {
    pub fn from(config_path: &str) -> Result<Config, String> {
        let config_file = read_toml(config_path)?;

        // Database
        let database = match config_file.get("Database") {
            None => return Err("`Database` not found in config file".to_string()),
            Some(database) => database,
        };
        let db_path = match database.get("db_path") {
            None => return Err("missing filed `Database::db_path`".to_string()),
            Some(db_path) => db_path.as_str().to_owned().unwrap().to_string(),
        };

        // Task
        let task = match config_file.get("Task") {
            Some(task) => {
                let timeout = match task.get("timeout") {
                    Some(timeout) => {
                        if let Ok(timeout) = timeout.to_string().parse::<u64>() {
                            timeout
                        } else {
                            return Err("failed to parse `Task::timeout`".to_string());
                        }
                    }
                    None => {
                        return Err("missing field `Task::timeout` (type u64 required)".to_string());
                    }
                };
                Some(Task { timeout })
            }
            None => None,
        };

        // Mail
        let mail = match config_file.get("Mail") {
            Some(mail) => {
                let address = match mail.get("address") {
                    Some(address) => address.as_str().to_owned().unwrap().to_string(),
                    None => {
                        return Err("missing field `Mail::address`".to_string());
                    }
                };
                let password = match mail.get("password") {
                    Some(password) => password.as_str().to_owned().unwrap().to_string(),
                    None => {
                        return Err("missing field `Mail::password`".to_string());
                    }
                };
                Some(Mail { address, password })
            }
            None => {
                log::warn!("mail not config");
                None
            }
        };

        Ok(Config {
            db_path,
            task,
            mail,
        })
    }
}

fn read_toml(file_path: &str) -> Result<Value, String> {
    if let Ok(file_contents) = fs::read_to_string(file_path) {
        if let Ok(toml_value) = toml::from_str(&file_contents) {
            Ok(toml_value)
        } else {
            Err("failed to parse config file".to_string())
        }
    } else {
        // todo: creat template conf file
        Err("Config.toml not found".to_string())
    }
}

#[cfg(test)]
#[test]
fn read_config() {
    let cfg = Config::from("./Config.toml");
    print!("{:?}", cfg.unwrap());
}
