#[test]
fn test_build_db_connection_from_config() {
    use std::fs;

    use crate::config;
    use crate::task::task_list::TaskList;

    let tmp_db = tempfile::NamedTempFile::new().expect("Failed to create temporary database");
    let tmp_db_path = tmp_db.path().to_str().unwrap();

    println!("{}", &tmp_db_path);

    let toml_content = format!(
        r#"
    [database]
    db_path = "{}"

    [task]
    timeout = 5000
"#,
        tmp_db_path
    );
    let tmp_file = tempfile::NamedTempFile::new().expect("Failed to create temporary file");
    let tmp_file_path = tmp_file.path().to_str().unwrap();

    fs::write(&tmp_file, toml_content).expect("Failed to write temporary file");

    let config = config::from(tmp_file_path).expect("Failed to read configuration");

    let task_list = TaskList::from(&config.db_path).unwrap();
}
