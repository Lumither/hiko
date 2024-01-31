use std::error::Error;
use std::ops::Deref;
use std::process::exit;

use sqlx::{query, MySqlPool};

use crate::database::Database;

pub struct TaskDB {
    handler: MySqlPool,
}

impl Deref for TaskDB {
    type Target = MySqlPool;

    fn deref(&self) -> &Self::Target {
        &self.handler
    }
}

impl Database for TaskDB {
    type Database = TaskDB;
    async fn connect(
        url: String,
        usr: String,
        passwd: String,
    ) -> Result<Self::Database, Box<dyn Error>> {
        dbg!(format!("mysql://{}:{}@{}", usr, passwd, url));

        let conn = match MySqlPool::connect(format!("mysql://{}:{}@{}", usr, passwd, url).as_str())
            .await
        {
            Ok(conn) => conn,
            Err(e) => {
                log::error!("{}", e);
                exit(1)
            }
        };
        Ok(TaskDB { handler: conn })
    }

    async fn init(&self) -> Result<(), Box<dyn Error>> {
        query(
            r#"
            create table if not exists tasks
            (
                id          varchar(36),
                type        varchar(40),
                name        varchar(40),
                description varchar(200),
                fails       int default 0,
                args        json
            );
        "#,
        )
        .execute(&**self)
        .await?;
        Ok(())
    }

    // todo: update
    async fn insert(&self, data: serde_json::Value) -> Result<(), Box<dyn Error>> {
        let uuid = data["id"].clone().to_owned();
        let task_type = data["type"].clone().to_owned();
        let name = data["name"].as_str();
        let description = data["description"].as_str();
        let fails = match data["fails"].clone().as_i64() {
            None => 0u32,
            Some(cnt) => cnt as u32,
        };
        let args = data["args"].to_string();
        query(
            r#"insert into tasks value 
                (?, ?, ?, ?, ?, ?);
            "#,
        )
        .bind(uuid.as_str())
        .bind(task_type.as_str())
        .bind(name)
        .bind(description)
        .bind(fails)
        .bind(args)
        .execute(&**self)
        .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use uuid::Uuid;

    use crate::database::{tasks::TaskDB, Database};
    use crate::task::tasks::match_url_content::{Args, MatchUrlContent};
    use crate::task::Description;

    #[tokio::test]
    async fn test_connect() -> Result<(), Box<dyn Error>> {
        let db = TaskDB::connect(
            "localhost/test".to_string(),
            "test".to_string(),
            "test".to_string(),
        )
        .await?;
        db.init().await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_insert() -> Result<(), Box<dyn Error>> {
        let db = TaskDB::connect(
            "localhost/test".to_string(),
            "test".to_string(),
            "test".to_string(),
        )
        .await?;
        db.init().await?;

        let task = MatchUrlContent {
            id: Uuid::new_v4(),
            description: Some(Description {
                name: "name".to_string(),
                text: "description".to_string(),
            }),
            args: Args {
                url: "".to_string(),
                content: "".to_string(),
            },
        };
        db.insert(serde_json::to_value(task).unwrap()).await?;
        Ok(())
    }
}
