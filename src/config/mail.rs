use toml::Value;

use crate::config::ConfigError::MissingField;
use crate::config::{ConfigComponent, ConfigError};

#[derive(Debug, Clone)]
pub struct Mail {
    pub smtp_username: String,
    pub smtp_server: String,
    pub smtp_password: String,
    pub smtp_port: u16,
    pub target_email: String,
}

impl ConfigComponent for Mail {
    type ConfigType = Option<Mail>;

    fn parse(config_file: Value) -> Result<Self::ConfigType, ConfigError> {
        Ok(match config_file.get("Mail") {
            Some(mail) => {
                let smtp_username = match mail.get("smtp_username") {
                    Some(address) => address.as_str().to_owned().unwrap().to_string(),
                    None => {
                        return Err(MissingField("Mail::smtp_username".to_string()));
                    }
                };
                let smtp_password = match mail.get("smtp_password") {
                    Some(password) => password.as_str().to_owned().unwrap().to_string(),
                    None => {
                        return Err(MissingField("Mail::smtp_password".to_string()));
                    }
                };
                let smtp_server = match mail.get("smtp_server") {
                    Some(smtp_server) => smtp_server.as_str().to_owned().unwrap().to_string(),
                    None => {
                        return Err(MissingField("Mail::smtp_server".to_string()));
                    }
                };
                let smtp_port = match mail.get("smtp_port") {
                    Some(smtp_port) => {
                        if let Some(smtp_port) = smtp_port.as_integer() {
                            smtp_port as u16
                        } else {
                            587u16
                        }
                    }
                    None => 587u16,
                };
                let target_email = match mail.get("target_email") {
                    Some(target_email) => target_email.as_str().to_owned().unwrap().to_string(),
                    None => {
                        return Err(MissingField("Mail::target_email".to_string()));
                    }
                };
                Some(Mail {
                    smtp_username,
                    smtp_password,
                    smtp_server,
                    smtp_port,
                    target_email,
                })
            }
            None => {
                log::warn!("Mail not config");
                None
            }
        })
    }
}
