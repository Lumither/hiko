use std::error::Error;

use serde_json::Value;
use uuid::Uuid;

pub mod tasks;
mod utils;

pub trait Database {
    type Database;

    async fn connect(
        url: String,
        usr: String,
        passwd: String,
    ) -> Result<Self::Database, Box<dyn Error>>;

    async fn init(&self) -> Result<(), Box<dyn Error>>;

    async fn insert(&self, data: Value) -> Result<(), Box<dyn Error>>;

    async fn delete(&self, items: Vec<Uuid>) -> Result<(), Box<dyn Error>>;

    async fn read(&self, items: Vec<Uuid>) -> Vec<Result<Value, Box<dyn Error>>>;

    async fn query<'a>(&self, query: &'a str) -> Result<Vec<Value>, Box<dyn Error>>;

    async fn update(&self, items: Vec<(Uuid, Value)>) -> Result<(), Box<dyn Error>>;
}
