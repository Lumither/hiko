use std::error::Error;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::task::TaskError::RuntimeError;
use crate::task::{Description, Task};

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct Args {
    pub url: String,
    pub content: String,
}

impl Debug for Args {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MatchUrlContent {
    pub id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub description: Option<Description>,
    pub args: Args,
}

impl Deref for MatchUrlContent {
    type Target = Args;

    fn deref(&self) -> &Self::Target {
        &self.args
    }
}

impl Task for MatchUrlContent {
    async fn exec(&mut self) -> Result<(), Box<dyn Error>> {
        // todo: timeout
        match reqwest::get(&self.url).await {
            Ok(response) => {
                let res_content = response.text().await?;
                if res_content.contains(&self.content) {
                    Ok(())
                } else {
                    Err(Box::new(RuntimeError(
                        format!(
                            "Content Mismatch: {} ({} expected)",
                            res_content, &self.content
                        )
                        .to_string(),
                    )))
                }
            }
            Err(err) => Err(Box::new(RuntimeError(err.to_string()))),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use uuid::Uuid;

    use crate::task::tasks::match_url_content::{Args, MatchUrlContent};
    use crate::task::{Description, Task};

    #[test]
    fn test_serialize() {
        dbg!(serde_json::to_string(&MatchUrlContent {
            id: Uuid::new_v4(),
            description: Some(Description {
                name: "name".to_string(),
                text: "text".to_string(),
            }),
            args: Args {
                url: "url".to_string(),
                content: "content".to_string(),
            },
        })
        .unwrap());
    }

    #[test]
    fn test_deserialize() {
        let task = serde_json::from_str::<MatchUrlContent>(
            "{\"id\":\"5e07e449-0c61-4e5c-ad5b-97e3f1a563e5\",\"name\":\"name\",\"text\":\"text\",\"args\":{\"url\":\"url\",\"content\":\"content\"}}" 
        );
        dbg!(task.unwrap());

        let task = serde_json::from_str::<MatchUrlContent>(
            "{\"id\":\"5e07e449-0c61-4e5c-ad5b-97e3f1a563e5\",\"args\":{\"url\":\"url\",\"content\":\"content\"}}"
        );
        dbg!(task.unwrap());
    }

    #[tokio::test]
    async fn test_matching() -> Result<(), Box<dyn Error>> {
        MatchUrlContent {
            id: Uuid::new_v4(),
            description: Some(Description {
                name: "name".to_string(),
                text: "description".to_string(),
            }),
            args: Args {
                url: "https://example.com".to_string(),
                content: "example".to_string(),
            },
        }
        .exec()
        .await
    }

    #[tokio::test]
    async fn test_not_matching() {
        assert!(MatchUrlContent {
            id: Uuid::new_v4(),
            description: Some(Description {
                name: "name".to_string(),
                text: "description".to_string(),
            }),
            args: Args {
                url: "https://example.com".to_string(),
                content: "lol".to_string(),
            },
        }
        .exec()
        .await
        .is_err());
    }
}
