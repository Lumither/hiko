use std::error::Error;
use std::ops::Deref;
use std::process::exit;

use sqlx::{query, MySqlPool};

pub struct TaskDB {
    handler: MySqlPool,
}

impl Deref for TaskDB {
    type Target = MySqlPool;

    fn deref(&self) -> &Self::Target {
        &self.handler
    }
}

pub async fn connect(url: String, usr: String, passwd: String) -> Result<TaskDB, Box<dyn Error>> {
    dbg!(format!("mysql://{}:{}@{}", usr, passwd, url));

    let conn =
        match MySqlPool::connect(format!("mysql://{}:{}@{}", usr, passwd, url).as_str()).await {
            Ok(conn) => conn,
            Err(e) => {
                log::error!("{}", e);
                exit(1)
            }
        };
    Ok(TaskDB { handler: conn })
}

impl TaskDB {
    pub async fn init(&self) -> Result<(), Box<dyn Error>> {
        query(
            r#"
            create table if not exists tasks (
                id int
            );
        "#,
        )
        .execute(&**self)
        .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::database::tasks;

    #[tokio::test]
    async fn test_connect() -> Result<(), Box<dyn Error>> {
        let db = tasks::connect(
            "localhost/test".to_string(),
            "test".to_string(),
            "test".to_string(),
        )
        .await?;
        db.init().await?;
        Ok(())
    }
}
