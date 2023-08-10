use rusqlite;

use crate::task::TaskType;

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
        let stmt = self
            .db_connection
            .prepare("SELECT task_id, task_type FROM task_list");
        if let Ok(mut stmt) = stmt {
            let task_list = stmt
                .query_map([], |row| {
                    Ok(Box::new((
                        uuid::Uuid::parse_str(row.get::<_, String>(0)?.as_str()).unwrap(),
                        serde_json::from_str::<TaskType>(row.get::<_, String>(1)?.as_str())
                            .unwrap(),
                    )))
                })?
                .collect::<Result<Vec<_>, _>>()?;
            todo!("check list");
            Ok(())
        } else {
            Err(stmt.unwrap_err())
        }
    }
}
