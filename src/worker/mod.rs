use std::sync::Arc;

use crate::config::Config;
use crate::database::tasks::TaskDB;
use crate::mail::Mailer;
use crate::worker::executor::task_executor;
use crate::worker::guardian::guardian;

mod executor;
mod guardian;
mod utils;

pub async fn run(config: Config, task_db: Arc<TaskDB>) {
    let mailer = Mailer::new(config.mail);

    let executor = tokio::spawn(task_executor(
        task_db.clone(),
        config.general.task_refresh_rate,
    ));
    let guardian = tokio::spawn(guardian(
        task_db.clone(),
        mailer,
        config.general.notification_refresh_rate,
    ));

    let _ = tokio::join!(guardian, executor);
}
