use std::error::Error;

use serde_json::Value;
use sqlx::mysql::MySqlArguments;
use sqlx::MySql;
use sqlx_core::query::Query;

pub mod record;
pub mod tasks;
pub mod utils;

pub trait Database {
    type Database;

    async fn connect(url: &str, usr: &str, passwd: &str) -> Result<Self::Database, Box<dyn Error>>;

    async fn init(&self) -> Result<(), Box<dyn Error>>;

    async fn query<'a>(
        &self,
        query: Query<'a, MySql, MySqlArguments>,
    ) -> Result<Vec<Result<Value, Box<dyn Error>>>, Box<dyn Error>>;
}
