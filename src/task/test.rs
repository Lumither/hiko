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
#[should_panic]
async fn test_expired_tls() {
    let mut task = Task::new(
        String::from("Task 2"),
        String::from("https://expired.badssl.com/"),
        String::from("Check page status"),
        TaskType::CheckReturnCode(200),
    );

    task.update().await;
    println!("{:?}\n", task.task_status);
    assert_eq!(
        task.task_status,
        TaskStatus::ERR(String::from("Status Code Mismatch"))
    );
}

#[tokio::test]
async fn test_check_return_code() {
    let task = Task::new(
        String::from("Task 3"),
        String::from("https://example.com"),
        String::from("Check page status"),
        TaskType::CheckReturnCode(200),
    );

    let result = task.trace().await;

    assert_eq!(result, Ok(()));
}

#[tokio::test]
async fn test_match_url_content_success() {
    let task = Task::new(
        String::from("Task 4"),
        String::from("https://example.com"),
        String::from("Check if content match"),
        TaskType::MatchUrlContent("example".to_string()),
    );

    let result = task.trace().await;

    assert_eq!(result, Ok(()));
}
