use serde::{Deserialize, Serialize};

mod tasks;

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

// todo: new error trait, impl error
