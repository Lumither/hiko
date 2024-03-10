use crate::database::tasks::TaskDB;
use crate::mail::Mailer;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

pub async fn guardian(task_db: Arc<TaskDB>, mailer: Mailer, notification_refresh_rate: u64) {
    loop {
        dbg!("send");
        sleep(Duration::from_secs(notification_refresh_rate)).await;
    }
}
