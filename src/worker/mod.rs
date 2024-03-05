use std::sync::Arc;

use crate::config::Config;
use crate::database::tasks::TaskDB;
use crate::worker::executor::task_executor;

mod executor;
mod utils;

pub async fn run(config: Config, task_db: Arc<TaskDB>) {
    let executor = tokio::spawn(task_executor(task_db.clone()));

    let _ = tokio::join!(executor);
}
