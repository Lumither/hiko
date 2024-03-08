use std::sync::Arc;

use crate::config::Config;
use crate::database::tasks::TaskDB;
use crate::worker::executor::task_executor;
use crate::worker::guardian::guardian;

mod executor;
mod guardian;
mod utils;

pub async fn run(config: Config, task_db: Arc<TaskDB>) {
    let executor = tokio::spawn(task_executor(task_db.clone()));
    let guardian = tokio::spawn(guardian());

    let _ = tokio::join!(guardian, executor);
}
