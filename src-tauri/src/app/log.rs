use crate::extensions::AppHandleExt as _;

use super::App;
use anyhow::{Context as _, Result};
use chrono::{DateTime, Local};
use fs_extra::dir::create_all;
use std::fmt::{self, Display};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

const DEFAULT_TIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

#[derive(Debug, Clone)]
pub struct Log {
    pub level: Level,
    pub data: Data,
}

#[derive(Debug, Clone)]
pub struct Data {
    pub time: DateTime<Local>,
    pub message: String,
}

#[derive(Debug, Clone)]
pub enum Level {
    Warn,
    Info,
    Error,
    Success,
}

impl Log {
    fn new(level: Level, message: impl Into<String>) -> Self {
        let data = Data {
            time: Local::now(),
            message: message.into(),
        };
        Self { level, data }
    }

    pub fn info(message: impl Into<String>) -> Self {
        Self::new(Level::Info, message)
    }

    pub fn warn(message: impl Into<String>) -> Self {
        Self::new(Level::Warn, message)
    }

    pub fn success(message: impl Into<String>) -> Self {
        Self::new(Level::Success, message)
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self::new(Level::Error, message)
    }

    fn format(&self, time_format: &str) -> String {
        let time_str = self.data.time.format(time_format).to_string();
        format!("[{}] {} {}", self.level, time_str, self.data.message)
    }

    pub fn write_to_file(&self, file_path: impl AsRef<Path>) -> Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_path.as_ref())
            .with_context(|| format!("Failed to open log file: {:?}", file_path.as_ref()))?;
        writeln!(file, "{}", self.format("%Y-%m-%d %H:%M:%S"))?;
        Ok(())
    }

    pub fn write_to_file_with_format(
        &self,
        file_path: impl AsRef<Path>,
        time_format: &str,
    ) -> Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_path.as_ref())
            .with_context(|| format!("Failed to open log file: {:?}", file_path.as_ref()))?;
        writeln!(file, "{}", self.format(time_format))?;
        Ok(())
    }

    pub fn save(&self) -> Result<()> {
        let log_name = format!("{}.log", self.data.time.format("%Y-%m-%d"));
        let log_dir = App::get_resource_dir().join("log");
        create_all(&log_dir, false)?;
        let log_path = log_dir.join(&log_name);
        self.write_to_file(&log_path)
    }

    pub fn send_to_app_log(&self) {
        let app_handle = App::get_app_handle();
        let target = match self.level {
            Level::Warn => "log:warn",
            Level::Info => "log:info",
            Level::Error => "log:error",
            Level::Success => "log:success",
        };
        app_handle.emit_with_timestamp(target, &self.data.message);
    }

    pub fn send_to_app_msg(&self) {
        let app_handle = App::get_app_handle();

        if let Err(error) = self.save() {
            app_handle
                .emit_with_timestamp("msg:error", &format!("Save error log failed. ({})", error));
        }

        let target = match self.level {
            Level::Warn => "msg:warn",
            Level::Info => "msg:info",
            Level::Error => "msg:error",
            Level::Success => "msg:success",
        };
        app_handle.emit_with_timestamp(target, &self.data.message);
    }
}

impl Display for Log {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format(DEFAULT_TIME_FORMAT))
    }
}

impl Level {
    fn as_str(&self) -> &'static str {
        match self {
            Level::Info => "INFO",
            Level::Warn => "WARN",
            Level::Error => "ERROR",
            Level::Success => "SUCCESS",
        }
    }
}

impl Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
