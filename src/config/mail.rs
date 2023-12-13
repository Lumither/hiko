use crate::config::ConfigError::MissingField;
use crate::config::{ConfigComponent, ConfigError};
use toml::Value;

#[derive(Debug)]
pub struct Mail {
    pub address: String,
    pub smtp_server: String,
    pub password: String,
}

impl ConfigComponent for Mail {
    type ConfigType = Option<Mail>;

    fn parse(config_file: Value) -> Result<Self::ConfigType, ConfigError> {
        Ok(match config_file.get("Mail") {
            Some(mail) => {
                let address = match mail.get("address") {
                    Some(address) => address.as_str().to_owned().unwrap().to_string(),
                    None => {
                        return Err(MissingField("Mail::address".to_string()));
                    }
                };
                let password = match mail.get("password") {
                    Some(password) => password.as_str().to_owned().unwrap().to_string(),
                    None => {
                        return Err(MissingField("Mail::password".to_string()));
                    }
                };
                let smtp_server = match mail.get("smtp_server") {
                    Some(smtp_server) => smtp_server.as_str().to_owned().unwrap().to_string(),
                    None => {
                        return Err(MissingField("Mail::smtp_server".to_string()));
                    }
                };
                Some(Mail {
                    address,
                    password,
                    smtp_server,
                })
            }
            None => {
                log::warn!("Mail not config");
                None
            }
        })
    }
}
