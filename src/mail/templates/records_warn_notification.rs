use serde_json::Value;

pub fn records_warn_notification(record_list: Vec<Value>) -> String {
    // todo: style
    let mut message: String = r#"
    Error(s) found during executing task(s):
    err_id: task_id(time): message
    "#
    .to_string();
    for record in record_list {
        message += format!(
            "{}: {}({}): {}\n",
            &record["err_id"], &record["task_id"], &record["time"], &record["message"]
        )
        .as_str();
    }
    message
}
