use std::process::exit;
use std::sync::Arc;

use sqlx::query;
use uuid::Uuid;

use crate::database::record::RecordDB;
use crate::database::tasks::TaskDB;
use crate::database::Database;

pub async fn add_fails(
    task_db: Arc<TaskDB>,
    records_database: Arc<RecordDB>,
    id: Uuid,
    error_message: String,
) {
    // push error to db records table
    match records_database
        .query(
            query(
                "
                        insert records value (?, ?, now(), ?, 0);
                    ",
            )
            .bind(Uuid::new_v4().to_string())
            .bind(id.to_string())
            .bind(error_message),
        )
        .await
    {
        Ok(_) => (),
        Err(e) => {
            log::error!(
                "fatal error at pushing task error into records table`{}`: {}",
                id,
                e.to_string()
            );
            exit(1);
        }
    }

    // add fail count
    match task_db
        .query(
            query(
                "
                        UPDATE tasks
                        SET fails = fails + 1
                        WHERE id = ?;
                    ",
            )
            .bind(id.to_string()),
        )
        .await
    {
        Ok(_) => (),
        Err(e) => {
            log::error!(
                "fatal error at updating task info to tasks database: `{}`: {}",
                id,
                e.to_string()
            );
            exit(1);
        }
    }
}
