use std::fs::OpenOptions;
use std::io::{stdout, Write};

use chrono::Local;
use fern::Dispatch;
use log::LevelFilter;
pub use log::{debug, error, info, trace, warn};

pub fn init(log_path: String) {
    let mut log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path)
        .unwrap_or_else(|e| panic!("[fatal] Failed to open/create log file: {}", e));

    writeln!(&mut log_file).expect("[fatal] Unable to write log file");

    Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                message
            ))
        })
        .level(LevelFilter::Info)
        .chain(stdout())
        .chain(log_file)
        .apply()
        .unwrap();
}
