#[test]
fn test_from_valid() {
    use std::fs;

    use crate::config::from;

    let toml_content = r#"
            [database]
            db_path = "/path/to/database.db"

            [task]
            timeout = 5000
        "#;
    let temp_file = tempfile::NamedTempFile::new().expect("Failed to create temporary file");
    let file_path = temp_file.path().to_str().unwrap();

    fs::write(&temp_file, toml_content).expect("Failed to write temporary file");

    let config = from(file_path).expect("Failed to read configuration");

    assert_eq!(config.db_path, "/path/to/database.db");
    assert_eq!(config.timeout, 5000);
}

#[test]
fn test_from_missing_db_path() {
    use std::fs;

    use crate::config::from;

    let toml_content = r#"
            [database]
            
            [task]
            timeout = 5000
        "#;
    let temp_file = tempfile::NamedTempFile::new().expect("Failed to create temporary file");
    let file_path = temp_file.path().to_str().unwrap();

    fs::write(&temp_file, toml_content).expect("Failed to write temporary file");

    let result = from(file_path);

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        "Missing 'db_path' in configuration file"
    );
}

#[test]
fn test_from_missing_timeout() {
    use std::fs;

    use crate::config::from;

    let toml_content = r#"
            [database]
            db_path = "/path/to/database.db"

            [task]
        "#;
    let temp_file = tempfile::NamedTempFile::new().expect("Failed to create temporary file");
    let file_path = temp_file.path().to_str().unwrap();

    fs::write(&temp_file, toml_content).expect("Failed to write temporary file");

    let result = from(file_path);

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        "Missing 'timeout' in configuration file"
    );
}
