use std::error::Error;
use std::fmt::{Debug, Formatter};
use std::future::Future;
use std::ops::Deref;
use std::pin::Pin;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::task::TaskError::RuntimeError;
use crate::task::{Description, Task};

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct Args {
    pub url: String,
    pub code: u16,
}

impl Debug for Args {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub struct CheckReturnCode {
    pub id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub description: Option<Description>,
    pub args: Args,
}

impl Deref for CheckReturnCode {
    type Target = Args;

    fn deref(&self) -> &Self::Target {
        &self.args
    }
}

impl Task for CheckReturnCode {
    fn exec(&mut self) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn Error>>> + Send + '_>> {
        // todo: timeout
        Box::pin(async {
            let response = reqwest::get(&self.url).await?;
            let res_code = response.status().as_u16();
            if res_code.eq(&self.code) {
                Ok(())
            } else {
                Err(Box::new(RuntimeError(
                    format!("Code Mismatch: {} ({} expected)", res_code, &self.code).to_string(),
                )) as Box<dyn Error>)
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::task::tasks::check_return_code::{Args, CheckReturnCode};
    use crate::task::{Description, Task};

    #[tokio::test]
    async fn test_matching() {
        CheckReturnCode {
            id: Uuid::new_v4(),
            description: Some(Description {
                name: "name".to_string(),
                text: "description".to_string(),
            }),
            args: Args {
                url: "https://example.com/a".to_string(),
                code: 404,
            },
        }
        .exec()
        .await
        .unwrap();
    }

    #[test]
    fn test_serialize() {
        dbg!(serde_json::to_string(&CheckReturnCode {
            id: Uuid::new_v4(),
            description: Some(Description {
                name: "name".to_string(),
                text: "text".to_string(),
            }),
            args: Args {
                url: "".to_string(),
                code: 0,
            },
        })
        .unwrap());
    }

    #[test]
    fn test_deserialize() {
        let task = serde_json::from_str::<CheckReturnCode>(
            "{\"type\":\"CheckReturnCode\",\"id\":\"b36edac2-c8c7-42cd-acd5-9afe7e7afa35\",\"args\":{\"url\":\"\",\"code\":0}}",
        );
        dbg!(task.unwrap());

        let task = serde_json::from_str::<CheckReturnCode>(
            "{\"type\":\"CheckReturnCode\",\"id\":\"5ab52418-240d-467b-a41e-4ee778fc276c\",\"name\":\"name\",\"text\":\"text\",\"args\":{\"url\":\"\",\"code\":0}}"
        );
        dbg!(task.unwrap());
    }
}
