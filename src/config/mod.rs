use std::fs;

use toml::Value;

mod test_config;

#[derive(Debug)]
pub struct Config {
    pub db_path: String,
    pub timeout: u64,
}

pub fn from(file_path: &str) -> Result<Config, String> {
    // Read the contents of the file
    let file_content =
        fs::read_to_string(file_path).map_err(|err| format!("Failed to read file: {}", err))?;

    let toml_value = file_content
        .parse::<Value>()
        .map_err(|err| format!("Failed to parse TOML: {}", err))?;

    let db_path = toml_value
        .get("Database")
        .and_then(|db| db.get("db_path"))
        .and_then(|path| path.as_str())
        .map(|path| path.to_owned())
        .ok_or_else(|| String::from("Missing 'db_path' in configuration file"))?;

    let timeout = toml_value
        .get("Task")
        .and_then(|task| task.get("timeout"))
        .and_then(|val| val.as_integer())
        .map(|val| val as u64)
        .ok_or_else(|| String::from("Missing 'timeout' in configuration file"))?;

    Ok(Config { db_path, timeout })
}
