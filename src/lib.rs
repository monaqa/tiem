use std::path::Path;

use anyhow::*;
use files::{LogDir, StatusFile};

mod files;

pub struct Tiem {
    status_file: StatusFile,
    log_dir: LogDir,
}

impl Tiem {
    pub fn load(status: &Path, log: &Path) -> Result<Self> {
        let status = StatusFile::open(status)?;
        let log = LogDir::open(log)?;
        Ok(Self {
            status_file: status,
            log_dir: log,
        })
    }

    /// タイマーをスタートさせる。
    /// もし既に何かが走っていたら終了する。
    pub fn start(&self) -> Result<()> {
        let status = &self.status_file;
        Ok(())
    }
}
