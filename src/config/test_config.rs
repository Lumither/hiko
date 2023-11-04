use crate::config::Task;

#[test]
fn test_from_valid() {
    use std::fs;

    use crate::config::from;

    let toml_content = r#"
            [Database]
            db_path = "/path/to/Database.Db"

            [Task]
            timeout = 5000
        "#;
    let temp_file = tempfile::NamedTempFile::new().expect("Failed to create temporary file");
    let file_path = temp_file.path().to_str().unwrap();

    fs::write(&temp_file, toml_content).expect("Failed to write temporary file");

    let config = from(file_path).expect("Failed to read configuration");

    assert_eq!(config.db_path, "/path/to/Database.Db");
    assert_eq!(config.task, Some(Task { timeout: 5000 }));
}

#[test]
fn test_from_missing_db_path() {
    use std::fs;

    use crate::config::from;

    let toml_content = r#"
            [Database]
            
            [Task]
            timeout = 5000
        "#;
    let temp_file = tempfile::NamedTempFile::new().expect("Failed to create temporary file");
    let file_path = temp_file.path().to_str().unwrap();

    fs::write(&temp_file, toml_content).expect("Failed to write temporary file");

    let result = from(file_path);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "missing filed `Database::db_path`");
}

#[test]
fn test_from_missing_timeout() {
    use std::fs;

    use crate::config::from;

    let toml_content = r#"
            [Database]
            db_path = "/path/to/Database.Db"

            [Task]
        "#;
    let temp_file = tempfile::NamedTempFile::new().expect("Failed to create temporary file");
    let file_path = temp_file.path().to_str().unwrap();

    fs::write(&temp_file, toml_content).expect("Failed to write temporary file");

    let result = from(file_path);

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        "missing field `Task::timeout` (type u64 required)"
    );
}
