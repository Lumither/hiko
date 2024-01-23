use std::fmt::{Debug, Formatter};
use std::ops::Deref;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::task::{Description, Task};

#[derive(Clone, Serialize, Deserialize)]
pub struct Args {
    pub url: String,
    pub code: u16,
}

impl Debug for Args {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckReturnCode {
    pub id: Uuid,
    pub description: Description,
    pub fails: u32,

    pub args: Args,
}

impl Deref for CheckReturnCode {
    type Target = Args;

    fn deref(&self) -> &Self::Target {
        &self.args
    }
}

impl Task for CheckReturnCode {
    async fn exec(&mut self) -> Result<(), String> {
        // todo: timeout
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
mod tests {
    use uuid::Uuid;

    use crate::task::tasks::check_return_code::{Args, CheckReturnCode};
    use crate::task::{Description, Task};

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
                args: Args {
                    url: "https://example.com/a".to_string(),
                    code: 404,
                },
            }
            .exec()
            .await,
            Ok(())
        );
    }
}
