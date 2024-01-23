use std::fmt::{Debug, Formatter};
use std::ops::Deref;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
    pub description: Description,
    pub fails: u32,

    pub args: Args,
}

impl Deref for MatchUrlContent {
    type Target = Args;

    fn deref(&self) -> &Self::Target {
        &self.args
    }
}

impl Task for MatchUrlContent {
    async fn exec(&mut self) -> Result<(), String> {
        // todo: timeout
        match reqwest::get(&self.url).await {
            Ok(response) => {
                let res_content = response.text().await;
                if let Ok(res_content) = res_content {
                    if res_content.contains(&self.content) {
                        Ok(())
                    } else {
                        Err(format!(
                            "Content Mismatch: \n\texpected \"{}\", found \"{}\"", // todo: modify style
                            &self.content, res_content
                        ))
                    }
                } else {
                    self.fails += 1;
                    Err(res_content.unwrap_err().to_string())
                }
            }
            Err(err) => {
                self.fails += 1;
                Err(err.to_string())
            }
        }
    }

    fn fail_count(&self) -> u32 {
        self.fails
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::task::tasks::match_url_content::{Args, MatchUrlContent};
    use crate::task::{Description, Task};

    #[test]
    fn test_serialization() {
        print!(
            "{}",
            serde_json::to_string(&MatchUrlContent {
                id: Uuid::new_v4(),
                description: Description {
                    name: "name".to_string(),
                    text: "description".to_string(),
                },
                fails: 0,

                args: Args {
                    url: "".to_string(),
                    content: "".to_string(),
                },
            })
            .unwrap()
        );
    }

    #[tokio::test]
    async fn test_matching() {
        assert_eq!(
            MatchUrlContent {
                id: Uuid::new_v4(),
                description: Description {
                    name: "name".to_string(),
                    text: "description".to_string(),
                },
                fails: 0,
                args: Args {
                    url: "https://example.com".to_string(),
                    content: "example".to_string(),
                },
            }
            .exec()
            .await,
            Ok(())
        );
    }

    #[tokio::test]
    async fn test_not_matching() {
        assert_ne!(
            MatchUrlContent {
                id: Uuid::new_v4(),
                description: Description {
                    name: "name".to_string(),
                    text: "description".to_string(),
                },
                fails: 0,
                args: Args {
                    url: "https://example.com".to_string(),
                    content: "lol".to_string(),
                },
            }
            .exec()
            .await,
            Ok(())
        );
    }
}
