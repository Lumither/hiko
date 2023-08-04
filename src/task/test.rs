use crate::task::Task;
#[cfg(test)]
use crate::task::{TaskStatus, TaskType};

#[tokio::test]
async fn test_update_normal() {
    let mut task = Task::new(
        String::from("Task 1"),
        String::from("https://example.com"),
        String::from("Check page status"),
        TaskType::CheckReturnCode(200),
    );

    task.update().await;

    assert_eq!(task.task_status, TaskStatus::Normal);
}

#[tokio::test]
async fn test_update_err() {
    let mut task = Task::new(
        String::from("Task 2"),
        String::from("https://example.com/notfound"),
        String::from("Check page status"),
        TaskType::CheckReturnCode(200),
    );

    task.update().await;

    assert_eq!(
        task.task_status,
        TaskStatus::ERR(String::from("Status Code Mismatch"))
    );
}

#[tokio::test]
async fn test_trace_check_return_code() {
    let task = Task::new(
        String::from("Task 3"),
        String::from("https://example.com"),
        String::from("Check page status"),
        TaskType::CheckReturnCode(200),
    );

    let result = task.trace().await;

    assert_eq!(result, Ok(()));
}
