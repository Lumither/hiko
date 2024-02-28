use std::error::Error;
use std::ops::Deref;
use std::process::exit;
use std::str::FromStr;

use serde_json::Value;
use sqlx::mysql::MySqlArguments;
use sqlx::{query, MySql, MySqlPool};
use sqlx_core::query::Query;
use uuid::Uuid;

use crate::database::utils::query_as_json;
use crate::database::Database;
use crate::task::tasks::check_return_code::CheckReturnCode;
use crate::task::tasks::match_url_content::MatchUrlContent;
use crate::task::Task;

pub struct TaskDB {
    handle: MySqlPool,
}

impl TaskDB {
    pub fn decode_json_list(list: Vec<Value>) -> Vec<Option<Box<dyn Task>>> {
        list.into_iter()
            .map(|task_jason| -> Option<Box<dyn Task>> {
                match task_jason["type"].as_str()? {
                    "MatchUrlContent" => Some(Box::new(MatchUrlContent {
                        id: Uuid::from_str(task_jason["id"].as_str()?).ok()?,
                        description: None,
                        args: serde_json::from_value(task_jason["args"].clone()).ok()?,
                    }) as Box<dyn Task>),
                    "CheckReturnCode" => Some(Box::new(CheckReturnCode {
                        id: Uuid::from_str(task_jason["id"].as_str()?).ok()?,
                        description: None,
                        args: serde_json::from_value(task_jason["args"].clone()).ok()?,
                    }) as Box<dyn Task>),
                    _ => None,
                }
            })
            .collect()
    }
}

impl Deref for TaskDB {
    type Target = MySqlPool;

    fn deref(&self) -> &Self::Target {
        &self.handle
    }
}

impl Database for TaskDB {
    type Database = TaskDB;
    async fn connect(
        url: String,
        usr: String,
        passwd: String,
    ) -> Result<Self::Database, Box<dyn Error>> {
        let conn = match MySqlPool::connect(format!("mysql://{}:{}@{}", usr, passwd, url).as_str())
            .await
        {
            Ok(conn) => conn,
            Err(e) => {
                log::error!("{}", e);
                exit(1)
            }
        };
        Ok(TaskDB { handle: conn })
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
    async fn insert(&self, data: Value) -> Result<(), Box<dyn Error>> {
        let uuid = data["id"].clone().to_owned();
        let task_type = data["type"].clone().to_owned();
        let name = data["name"].as_str().unwrap_or("");
        let description = data["description"].as_str().unwrap_or("");
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

    async fn query<'a>(
        &self,
        query: Query<'a, MySql, MySqlArguments>,
    ) -> Result<Vec<Result<Value, Box<dyn Error>>>, Box<dyn Error>> {
        query_as_json(self, query).await
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use serde_json::Value;
    use sqlx::query;
    use uuid::Uuid;

    use crate::database::utils::query_as_json;
    use crate::database::{tasks::TaskDB, Database};
    use crate::task::tasks::check_return_code::CheckReturnCode;
    use crate::task::tasks::match_url_content::MatchUrlContent;
    use crate::task::tasks::{check_return_code, match_url_content};
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

        for _ in 0..10 {
            let task_match_url_content = MatchUrlContent {
                id: Uuid::new_v4(),
                description: Some(Description {
                    name: "name".to_string(),
                    text: "description".to_string(),
                }),
                args: match_url_content::Args {
                    url: "".to_string(),
                    content: "".to_string(),
                },
            };

            let task_check_return_code = CheckReturnCode {
                id: Uuid::new_v4(),
                description: Some(Description {
                    name: "name".to_string(),
                    text: "description".to_string(),
                }),
                args: check_return_code::Args {
                    url: "".to_string(),
                    code: 200,
                },
            };

            db.insert(serde_json::to_value(task_match_url_content.clone()).unwrap())
                .await?;
            db.insert(serde_json::to_value(task_check_return_code.clone()).unwrap())
                .await?;
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_select() -> Result<(), Box<dyn Error>> {
        let db = TaskDB::connect(
            "localhost/test".to_string(),
            "test".to_string(),
            "test".to_string(),
        )
        .await?;
        db.init().await?;
        let res = query_as_json(&db, query("select * from tasks")).await;
        dbg!(res).unwrap();
        Ok(())
    }

    #[tokio::test]
    async fn test_request() -> Result<(), Box<dyn Error>> {
        let db = TaskDB::connect(
            "localhost/test".to_string(),
            "test".to_string(),
            "test".to_string(),
        )
        .await?;
        db.init().await?;

        let res = query_as_json(&db, query("select * from tasks")).await?;
        let res: Vec<Value> = res.into_iter().map(|json| json.unwrap()).collect();

        for task in TaskDB::decode_json_list(res) {
            println!("{:#?}", task.unwrap());
        }
        Ok(())
    }
}
