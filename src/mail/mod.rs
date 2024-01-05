use std::error::Error;

use lettre::transport::smtp::authentication::{Credentials, Mechanism};
use lettre::{Message, SmtpTransport, Transport};

use crate::config;
use crate::config::mail::Mail;

pub struct Mailer {
    attrib: Mail,
    creds: Credentials,
}

impl Mailer {
    fn new(mail: config::mail::Mail) -> Self {
        Mailer {
            attrib: mail.clone(),
            creds: Credentials::new(mail.smtp_username, mail.smtp_password),
        }
    }

    async fn send(&self, subject: String, body: String) -> Result<(), Box<dyn Error>> {
        let transport = SmtpTransport::starttls_relay(&self.attrib.smtp_server)?
            .credentials(self.creds.clone())
            .port(self.attrib.smtp_port)
            .authentication(vec![Mechanism::Login])
            .build();

        let email = Message::builder()
            .from(self.attrib.smtp_username.parse()?)
            .to(self.attrib.target_email.parse()?)
            .subject(subject)
            .body(body)?;

        transport.send(&email)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::config::mail::Mail;
    use crate::config::ConfigComponent;
    use crate::mail::Mailer;

    #[tokio::test]
    async fn test_send_email() -> Result<(), Box<dyn Error>> {
        let config_file = crate::config::read_toml("./confidential/test_mail.toml")?;

        let handler = Mailer::new(Mail::parse(config_file.clone())?.unwrap());

        handler
            .send("test subject".to_string(), "test body".to_string())
            .await
    }
}
