pub mod test;

#[derive(Debug)]
pub struct Task {
    task_name: String,
    task_url: String,
    task_description: String,
    task_type: TaskType,
    pub task_status: TaskStatus,
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

    pub async fn update(&mut self) {
        self.task_status = match self.trace().await {
            Ok(_) => TaskStatus::Normal,
            Err(err_msg) => TaskStatus::ERR(err_msg),
        };
    }

    pub async fn trace(&self) -> Result<(), String> {
        match &self.task_type {
            TaskType::CheckReturnCode(expected) => match reqwest::get(&self.task_url).await {
                Ok(responce) if responce.status().as_u16().eq(expected) => Ok(()),
                Ok(responce) => Err(String::from("Status Code Mismatch")),
                Err(err) => Err(err.to_string()),
            },
            TaskType::MatchUrlContent(expected) => {
                todo!("match url content");
            }
        }
    }
}
