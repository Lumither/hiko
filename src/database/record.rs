use std::error::Error;
use std::ops::Deref;

use serde_json::Value;
use sqlx::mysql::MySqlArguments;
use sqlx::{MySql, MySqlPool};
use sqlx_core::query::{query, Query};

use crate::database::utils::{get_db_handler, query_as_json};
use crate::database::Database;

pub struct RecordDB {
    handle: MySqlPool,
}

impl Deref for RecordDB {
    type Target = MySqlPool;

    fn deref(&self) -> &Self::Target {
        &self.handle
    }
}

impl Database for RecordDB {
    type Database = RecordDB;

    async fn connect(url: &str, usr: &str, passwd: &str) -> Result<Self::Database, Box<dyn Error>> {
        match get_db_handler(url, usr, passwd).await {
            Ok(conn) => Ok(RecordDB { handle: conn }),
            Err(e) => Err(e),
        }
    }

    async fn init(&self) -> Result<(), Box<dyn Error>> {
        query(
            r#"
            create table if not exists records
            (
                err_id      varchar(36),
                task_id     varchar(36),
                time        datetime,
                message     text,
                resolved    bool    default false
            );
        "#,
        )
        .execute(&**self)
        .await?;
        Ok(())
    }

    async fn query<'a>(
        &self,
        query: Query<'a, MySql, MySqlArguments>,
    ) -> Result<Vec<Result<Value, Box<dyn Error>>>, Box<dyn Error>> {
        query_as_json(self, query).await
    }
}

#[cfg(test)]
mod tests {
    use crate::config::Config;
    use crate::database::record::RecordDB;
    use crate::database::Database;
    use std::error::Error;

    #[tokio::test]
    async fn test_init() -> Result<(), Box<dyn Error>> {
        let config = Config::from("./confidential/config.toml").unwrap();
        let db = RecordDB::connect(
            &config.database.url,
            &config.database.user,
            &config.database.password,
        )
        .await?;
        db.init().await
    }
}
