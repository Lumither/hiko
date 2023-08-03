pub struct Task {
    task_name: String,
    task_url: String,
    task_description: String,
    task_type: TaskType,
    task_status: TaskStatus,
}

pub enum TaskType {
    CheckReturnCode(i32),
    MatchUrlContent(String),
}

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

    pub fn update(&mut self) {
        self.task_status = match self.trace() {
            Ok(_) => TaskStatus::Normal,
            Err(err_msg) => TaskStatus::ERR(err_msg),
        };
    }

    pub fn trace(&self) -> Result<(), String> {
        match &self.task_type {
            TaskType::CheckReturnCode(expected) => {
                todo!("check return code");
            }
            TaskType::MatchUrlContent(expected) => {
                todo!("match url content");
                Ok(())
            }
        }
    }
}
