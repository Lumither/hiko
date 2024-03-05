use std::sync::Arc;

use sqlx::query;
use uuid::Uuid;

use crate::database::tasks::TaskDB;
use crate::database::Database;

pub async fn add_fails(id: Uuid, task_db: Arc<TaskDB>) {
    let query_res = task_db
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
        .await;
    match query_res {
        Ok(_) => (),
        Err(e) => {
            log::error!("fatal error at task `{}`: {}", id, e);
        }
    }
}
