use rusqlite;

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

    pub fn load_tasks(&self) -> rusqlite::Result<()> {
        let buff = self.db_connection.prepare("SELECT * FROM task_list");
        if let Ok(mut buff) = buff {
            let task_list = buff.query_map([], |row| {
                Ok(Task::new(
                    row.get(0)?,
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                ))
            })?;
            Ok(())
        } else {
            Err(buff.unwrap_err())
        }
    }
}
