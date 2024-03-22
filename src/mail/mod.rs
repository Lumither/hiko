use std::error::Error;

use lettre::message::IntoBody;
use lettre::transport::smtp::authentication::{Credentials, Mechanism};
use lettre::{Message, SmtpTransport, Transport};

use crate::config::mail::Mail;

pub mod templates;

pub struct Mailer {
    attrib: Option<Mail>,
    creds: Option<Credentials>,
}

impl Mailer {
    pub fn new(mail: Option<Mail>) -> Self {
        if let Some(mail) = mail {
            Mailer {
                attrib: Some(mail.clone()),
                creds: Some(Credentials::new(mail.smtp_username, mail.smtp_password)),
            }
        } else {
            Mailer {
                attrib: None,
                creds: None,
            }
        }
    }

    pub async fn send<Subject, Body>(
        &self,
        subject: Subject,
        body: Body,
    ) -> Result<(), Box<dyn Error>>
    where
        Subject: Into<String>,
        Body: IntoBody,
    {
        if self.attrib.is_none() {
            return Ok(());
        }
        let transport = SmtpTransport::starttls_relay(&self.attrib.as_ref().unwrap().smtp_server)?
            .credentials(self.creds.clone().unwrap())
            .port(self.attrib.as_ref().unwrap().smtp_port)
            .authentication(vec![Mechanism::Login])
            .build();

        let email = Message::builder()
            .from(self.attrib.as_ref().unwrap().smtp_username.parse()?)
            .to(self.attrib.as_ref().unwrap().target_email.parse()?)
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

        let handler = Mailer::new(Mail::parse(config_file.clone())?);

        handler
            .send("test subject".to_string(), "test body".to_string())
            .await
    }
}
