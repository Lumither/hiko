use async_trait::async_trait;
use serde::{Deserialize, Serialize};

mod tasks;

#[async_trait]
pub trait Task {
    // do the task
    async fn exec(&mut self) -> Result<(), String>;

    fn fail_count(&self) -> u32;
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Description {
    name: String,
    text: String,
}
