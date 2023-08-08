use std::fmt;

pub mod task_list;
mod test_task;
mod test_task_list;

#[derive(Debug)]
pub struct Task {
    task_name: String,
    task_url: String,
    task_description: String,
    task_type: TaskType,
    task_status: TaskStatus,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TaskType {
    CheckReturnCode(u16),
    MatchUrlContent(String),
}

#[derive(Debug, PartialEq, Eq)]
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
            task_status: TaskStatus::UnTested,
        }
    }

    pub fn get_task_name(&self) -> String {
        self.task_name.clone()
    }

    pub fn get_task_url(&self) -> String {
        self.task_url.clone()
    }

    pub fn get_task_description(&self) -> String {
        self.task_description.clone()
    }

    pub fn get_task_type(&self) -> String {
        self.task_type.to_string()
    }

    pub async fn update(&mut self) {
        self.task_status = match self.trace().await {
            Ok(_) => TaskStatus::Normal,
            Err(err_msg) => TaskStatus::ERR(err_msg),
        };
    }

    pub async fn trace(&self) -> Result<(), String> {
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
        match self {
            TaskType::CheckReturnCode(expected) => write!(f, "CheckReturnCode({})", expected),
            TaskType::MatchUrlContent(expected) => write!(f, "MatchUrlContent({})", expected),
        }
    }
}
