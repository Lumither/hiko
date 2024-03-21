use std::sync::Arc;

use crate::config::Config;
use crate::database::record::RecordDB;
use crate::database::tasks::TaskDB;
use crate::mail::Mailer;
use crate::worker::executor::executor;
use crate::worker::guardian::guardian;

mod executor;
mod guardian;
mod utils;

pub async fn run(config: Config, tasks_database: Arc<TaskDB>, records_database: Arc<RecordDB>) {
    // mailer init
    let mailer = Mailer::new(config.mail);

    let executor = tokio::spawn(executor(
        tasks_database.clone(),
        records_database.clone(),
        config.general.task_refresh_rate,
    ));
    let guardian = tokio::spawn(guardian(
        tasks_database.clone(),
        records_database.clone(),
        mailer,
        config.general.notification_refresh_rate,
    ));

    let _ = tokio::join!(guardian, executor);
}
