use rusqlite;
use serde_json::from_str;
use tokio::{runtime::Runtime, try_join};

use crate::task::{TaskStatus, TaskType};

use super::Task;

#[derive(Debug)]
pub struct TaskList {
    db_connection: rusqlite::Connection,
}

impl TaskList {
    pub fn from(db_path: &str) -> Result<Self, String> {
        let tmp_db_conn = rusqlite::Connection::open(db_path);
        if let Ok(tmp_db_conn) = tmp_db_conn {
            Ok(TaskList {
                db_connection: tmp_db_conn,
            })
        } else {
            Err(tmp_db_conn.unwrap_err().to_string())
        }
    }

    pub fn execute(&self) -> rusqlite::Result<()> {
        let stmt = self.db_connection.prepare("SELECT * FROM task_list");
        if let Ok(mut stmt) = stmt {
            let tasks = stmt
                .query_map([], |row| {
                    Ok(Task {
                        task_name: row.get(0)?,
                        task_id: uuid::Uuid::parse_str(row.get::<_, String>(1)?.as_str()).unwrap(),
                        task_url: row.get(2)?,
                        task_description: row.get(3)?,
                        task_type: serde_json::from_str::<TaskType>(
                            row.get::<_, String>(4)?.as_str(),
                        )
                        .unwrap(),
                        task_status: serde_json::from_str::<TaskStatus>(
                            row.get::<_, String>(5)?.as_str(),
                        )
                        .unwrap(),
                        failure_count: from_str(row.get::<_, String>(6)?.trim()).unwrap(),
                    })
                })?
                .collect::<Result<Vec<_>, _>>()?;

            let mut async_runtime = Runtime::new().unwrap();
            let task_funtures = tasks.into_iter().map(|mut task| tokio::spawn(task.trace()));
            async_runtime.block_on(async {});
            todo!("asyc runtime");
            Ok(())
        } else {
            Err(stmt.unwrap_err())
        }
    }
}
