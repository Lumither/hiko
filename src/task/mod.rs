use std::fmt::{Debug, Formatter};

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

pub mod tasks;

pub trait Task: Serialize + DeserializeOwned {
    // do the task
    async fn exec(&mut self) -> Result<(), String>;

    fn fail_count(&self) -> u32;
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct Description {
    pub name: String,
    pub text: String,
}

impl Debug for Description {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

// todo: new error trait, impl error
