use std::process::exit;
use std::sync::Arc;
use std::time::Duration;

use futures::future::join_all;
use serde_json::Value;
use sqlx_core::query::query;
use tokio::task::JoinHandle;
use tokio::time::sleep;
use uuid::Uuid;

use crate::config::Config;
use crate::database::tasks::TaskDB;
use crate::database::utils::query_as_json;

pub async fn run(config: Config, task_db: Arc<TaskDB>) {
    loop {
        sleep(Duration::from_secs(config.general.refresh_rate)).await;

        let res = query_as_json(&task_db, query("select * from tasks"))
            .await
            .unwrap_or_else(|e| {
                log::error!("{}", e.to_string());
                exit(1);
            });
        let task_json_list: Vec<Value> = res
            .into_iter()
            .map(|json| {
                if let Err(e) = &json {
                    log::error!("{}", e);
                }
                json
            })
            .filter_map(|json| json.ok())
            .collect();
        let tasks: Vec<_> = TaskDB::decode_json_list(task_json_list)
            .into_iter()
            .map(|task| {
                if let Err(task_json) = &task {
                    log::error!("Error on parsing task: {:#?}", task_json);
                }
                task
            })
            .filter_map(|task| task.ok())
            .map(|task| {
                (
                    task.get_id(),
                    tokio::spawn(async move {
                        match task.exec().await {
                            Ok(_) => Ok(()),
                            Err(e) => Err(e.to_string()),
                        }
                    }),
                )
            })
            .collect();

        let (task_ids, task_futures): (Vec<Uuid>, Vec<JoinHandle<Result<(), String>>>) =
            tasks.into_iter().unzip();
        let results = join_all(task_futures).await;
        for (id, res) in task_ids.into_iter().zip(results.into_iter()) {
            match res {
                Ok(Ok(_)) => (),
                Ok(Err(e)) => todo!(),
                Err(_) => todo!(),
            }
        }

        todo!()
    }
}
