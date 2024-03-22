use serde_json::Value;
use std::process::exit;
use std::sync::Arc;
use std::time::Duration;

use crate::database::record::RecordDB;
use crate::database::tasks::TaskDB;
use crate::database::Database;
use crate::mail::templates::records_warn_notification::records_warn_notification;
use crate::mail::Mailer;
use sqlx_core::query::query;
use tokio::time::sleep;

pub async fn guardian(
    tasks_database: Arc<TaskDB>,
    records_database: Arc<RecordDB>,
    mailer: Mailer,
    notification_refresh_rate: u64,
) {
    loop {
        let query_res = records_database
            .query(query("select * from records where resolved = 0;"))
            .await
            .unwrap_or_else(|e| {
                log::error!("Query Error: {}", e);
                exit(1);
            });
        let fail_record_json_list: Vec<Value> = query_res
            .into_iter()
            .map(|json| {
                if let Err(e) = &json {
                    log::error!("Parse Error: {}", e);
                }
                json
            })
            .filter_map(|json| json.ok())
            .collect();

        match mailer
            .send(
                "[Hiko] Task Error Notification",
                records_warn_notification(fail_record_json_list),
            )
            .await
        {
            Ok(_) => {
                log::info!("Warning mail sent");
            }
            Err(e) => {
                log::error!("Mail sending failed: {}", e);
            }
        };

        sleep(Duration::from_secs(notification_refresh_rate)).await;
    }
}
