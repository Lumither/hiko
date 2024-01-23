use std::error::Error;

pub mod tasks;

pub trait Database {
    type Database;

    async fn connect(
        url: String,
        usr: String,
        passwd: String,
    ) -> Result<Self::Database, Box<dyn Error>>;
    async fn init(&self) -> Result<(), Box<dyn Error>>;
    async fn insert(&self, data: serde_json::Value) -> Result<(), Box<dyn Error>>;
}
