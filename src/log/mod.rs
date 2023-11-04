use std::io::Write;

use chrono::Local;
use env_logger::Builder;
pub use log::{debug, error, info, trace, warn};

pub fn init() {
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .init();
}
