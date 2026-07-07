use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;

pub struct Logger {
    file: File,
}

impl Logger {
    pub fn new(path: &Path) -> std::io::Result<Logger> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;

        Ok(Logger { file })
    }

    pub fn log(&mut self, msg: &str) {
        let timestamp = current_timestamp();
        let _ = writeln!(self.file, "[{}] {}", timestamp, msg);
    }
}

fn current_timestamp() -> String {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    format!("{}", now)
}
