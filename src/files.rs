use anyhow::*;
use chrono::{NaiveDateTime, NaiveTime};
use serde_with::{serde_as, DeserializeAs, SerializeAs};
use std::path::{Path, PathBuf};

use serde::*;

pub struct StatusFile(PathBuf);

/// status ファイルに載せる時刻。
struct StatusTime;

impl SerializeAs<NaiveDateTime> for StatusTime {
    fn serialize_as<S>(source: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = source.to_string();
        serializer.serialize_str(&s)
    }
}

impl<'de> DeserializeAs<'de, NaiveDateTime> for StatusTime {
    fn deserialize_as<D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer).map_err(serde::de::Error::custom)?;
        NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%S").map_err(serde::de::Error::custom)
    }
}

#[test]
fn test_serialize_status_time() {
    #[serde_as]
    #[derive(Deserialize, Serialize)]
    pub struct CustomTime {
        #[serde_as(as = "StatusTime")]
        time: NaiveDateTime,
    };

    let s = r#"{time: 2021-12-21T11:23:45}"#;
}

/// log ファイルに載せる時刻。
struct LogTime;

#[serde_as]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Status {
    task: String,
    #[serde_as(as = "StatusTime")]
    started: NaiveDateTime,
}

impl StatusFile {
    pub fn open(path: &Path) -> Result<StatusFile> {
        if !path.exists() {
            std::fs::write(path, "{}")?;
        }
        Ok(StatusFile(path.to_owned()))
    }

    pub fn get_status(&self) -> Result<Status> {
        todo!()
    }

    pub fn set_status(&self, status: &Status) -> Result<()> {
        todo!()
    }
}

pub struct LogDir(PathBuf);

pub struct LogRecord {
    task: String,
    started: NaiveTime,
    ended: NaiveTime,
}

pub struct LogFile(Vec<LogRecord>);

impl LogDir {
    pub fn open(path: &Path) -> Result<LogDir> {
        if !path.exists() {
            std::fs::create_dir_all(path)?;
        }
        Ok(LogDir(path.to_owned()))
    }
}
