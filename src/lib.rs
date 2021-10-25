use std::path::Path;

use anyhow::*;
use chrono::{NaiveDateTime, Utc};
use files::{LogDir, StatusFile};

mod files;

pub struct Tiem {
    status_file: StatusFile,
    log_dir: LogDir,
}

impl Tiem {
    pub fn load(status: &Path, log: &Path) -> Result<Self> {
        let status = StatusFile::new_file(status)?;
        let log = LogDir::new_file(log)?;
        Ok(Self {
            status_file: status,
            log_dir: log,
        })
    }

    /// タイマーをスタートさせる。
    /// もし既に何かが走っていたら終了する。
    pub fn start(&self, content: &str) -> Result<()> {
        let status_file = &self.status_file;
        if let Ok(files::Status::Running { task, started }) = status_file.get_status() {}
        let now = Utc::now().naive_local();
        status_file.set_status_running(content, &now)?;
        Ok(())
    }
}
