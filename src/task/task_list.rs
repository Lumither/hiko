use rusqlite;

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
}
