use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::task::{Description, Task};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MatchUrlContent {
    id: Uuid,
    description: Description,
    fails: u32,

    url: String,
    content: String,
}

#[async_trait]
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
            url: "".to_string(),
            content: "".to_string(),
        })
        .unwrap()
    );
}

#[cfg(test)]
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
            url: "https://example.com".to_string(),
            content: "example".to_string(),
        }
        .exec()
        .await,
        Ok(())
    );
}

#[cfg(test)]
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
            url: "https://example.com".to_string(),
            content: "lol".to_string(),
        }
        .exec()
        .await,
        Ok(())
    );
}
