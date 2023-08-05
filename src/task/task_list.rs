use rusqlite;

pub struct TaskList {
    db_connection: rusqlite::Connection,
}

impl TaskList {
    pub fn new() -> Self {
        todo!("under dev")
    }
}
