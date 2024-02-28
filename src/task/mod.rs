use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::future::Future;
use std::pin::Pin;

use serde::{Deserialize, Serialize};

pub mod tasks;

pub trait Task: Debug {
    fn exec(&mut self) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn Error>>> + Send + '_>>;
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

#[derive(Debug, Clone, PartialEq)]
pub enum TaskError {
    RuntimeError(String),
}

impl Display for TaskError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            TaskError::RuntimeError(s) => {
                write!(f, "Runtime Error: {}", s)
            }
        }
    }
}

impl Error for TaskError {}
