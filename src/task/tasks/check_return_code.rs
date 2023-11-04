use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::task::{Description, Task};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckReturnCode {
    id: Uuid,
    description: Description,
    fails: u32,

    url: String,
    code: u16,
}

#[async_trait]
impl Task for CheckReturnCode {
    async fn exec(&mut self) -> Result<(), String> {
        match reqwest::get(&self.url).await {
            Ok(response) => {
                let res_code = response.status().as_u16();
                if res_code.eq(&self.code) {
                    Ok(())
                } else {
                    Err(format!(
                        "Code Mismatch: \n\texpected \"{}\", found \"{}\"",
                        res_code, &self.code
                    ))
                }
            }
            Err(err) => Err(err.to_string()),
        }
    }

    fn fail_count(&self) -> u32 {
        self.fails
    }
}

#[cfg(test)]
#[tokio::test]
async fn test_matching() {
    assert_eq!(
        CheckReturnCode {
            id: Uuid::new_v4(),
            description: Description {
                name: "name".to_string(),
                text: "description".to_string(),
            },
            fails: 0,
            url: "https://example.com/a".to_string(),
            code: 404,
        }
        .exec()
        .await,
        Ok(())
    );
}
