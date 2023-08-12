use std::fmt;

use rusqlite::{
    types::{FromSql, ToSqlOutput},
    ToSql,
};
use serde::{Deserialize, Serialize};

pub mod task_list;
mod test_task;
mod test_task_list;

#[derive(Debug)]
pub struct Task {
    pub task_name: String,
    pub task_id: uuid::Uuid,
    pub task_url: String,
    pub task_description: String,
    pub task_type: TaskType,
    pub task_status: TaskStatus,
    pub failure_count: u16,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum TaskType {
    CheckReturnCode(u16),
    MatchUrlContent(String),
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum TaskStatus {
    UnTested,
    Normal,
    ERR(String),
}

impl Task {
    pub fn new(
        task_name: String,
        task_url: String,
        task_description: String,
        task_type: TaskType,
    ) -> Self {
        Task {
            task_name,
            task_url,
            task_description,
            task_type,
            task_id: uuid::Uuid::new_v4(),
            task_status: TaskStatus::UnTested,
            failure_count: 0,
        }
    }

    pub async fn update(&mut self) {
        self.task_status = match self.trace().await {
            Ok(_) => TaskStatus::Normal,
            Err(err_msg) => {
                // log
                self.failure_count += 1;
                TaskStatus::ERR(err_msg)
            }
        };
    }

    async fn trace(&self) -> Result<(), String> {
        match &self.task_type {
            TaskType::CheckReturnCode(expected) => match reqwest::get(&self.task_url).await {
                Ok(response) if response.status().as_u16().eq(expected) => Ok(()),
                Ok(_) => Err(String::from("Status Code Mismatch")),
                Err(err) => Err(err.to_string()),
            },
            TaskType::MatchUrlContent(expected) => match reqwest::get(&self.task_url).await {
                Ok(response) => {
                    let content = response.text().await;
                    if let Ok(content) = content {
                        if content.contains(expected) {
                            Ok(())
                        } else {
                            Err(String::from("Content Mismatch"))
                        }
                    } else {
                        Err(content.unwrap_err().to_string())
                    }
                }
                Err(err) => Err(err.to_string()),
            },
        }
    }
}

impl fmt::Display for TaskType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

impl FromSql for TaskType {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        Ok(serde_json::from_str(value.as_str()?).unwrap())
    }
}

impl ToSql for TaskType {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self.to_string()))
    }
}
