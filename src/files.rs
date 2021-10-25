use anyhow::*;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime, Timelike};
use itertools::Itertools;
use serde_with::{serde_as, DeserializeAs, SerializeAs};
use std::{
    convert::TryInto,
    fmt::Display,
    path::{Path, PathBuf},
    str::FromStr,
};

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

/// log ファイルに載せる時刻。
// struct LogTime;
//
// impl SerializeAs<NaiveTime> for LogTime {
//     fn serialize_as<S>(source: &NaiveTime, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         let s = source.format("%H:%M").to_string();
//         serializer.serialize_str(&s)
//     }
// }
//
// impl<'de> DeserializeAs<'de, NaiveTime> for LogTime {
//     fn deserialize_as<D>(deserializer: D) -> Result<NaiveTime, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         let s = String::deserialize(deserializer).map_err(serde::de::Error::custom)?;
//         NaiveTime::parse_from_str(&s, "%H:%M").map_err(serde::de::Error::custom)
//     }
// }

#[serde_as]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "kind", content = "content")]
pub enum Status {
    Running {
        task: String,
        #[serde_as(as = "StatusTime")]
        started: NaiveDateTime,
    },
    Stopped,
}

impl StatusFile {
    pub fn new_file(path: &Path) -> Result<StatusFile> {
        if !path.exists() {
            std::fs::write(path, "{}")?;
        }
        Ok(StatusFile(path.to_owned()))
    }

    pub fn get_status(&self) -> Result<Status> {
        let s = std::fs::read_to_string(&self.0)?;
        let status = serde_json::from_str(&s)?;
        Ok(status)
    }

    pub fn set_status_running(&self, content: &str, time: &NaiveDateTime) -> Result<()> {
        let status = Status::Running {
            task: content.to_owned(),
            started: time.to_owned(),
        };
        let status = serde_json::to_string(&status)?;
        std::fs::write(&self.0, status)?;
        Ok(())
    }

    pub fn set_status_stopped(&self) -> Result<()> {
        let status = Status::Stopped;
        let status = serde_json::to_string(&status)?;
        std::fs::write(&self.0, status)?;
        Ok(())
    }
}

pub struct LogDir(PathBuf);

pub struct LogRecord {
    task: String,
    started: NaiveTime,
    ended: NaiveTime,
}

impl Display for LogRecord {
    fn fmt(&self, f: &mut __private::Formatter<'_>) -> std::fmt::Result {
        let started = format!("{:02}:{:02}", self.started.hour(), self.started.minute());
        let ended = format!("{:02}:{:02}", self.ended.hour(), self.ended.minute());
        write!(f, "{}\t{}\t{}", started, ended, self.task)
    }
}

impl FromStr for LogRecord {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // let ss: Vec<&str> = s.split('\t').collect();
        // if ss.len() != 3 {
        //     return Err(anyhow!("# of rows does not match."));
        // }
        let [started, ended, text]: [&str; 3] = s
            .split('\t')
            .collect_vec()
            .try_into()
            .map_err(|_| anyhow!("# of rows does not match (expected 3).",))?;
        let started = NaiveTime::parse_from_str(started, "%H:%M")
            .map_err(|_| anyhow!("incorrect started time format."))?;
        let ended = NaiveTime::parse_from_str(ended, "%H:%M")
            .map_err(|_| anyhow!("incorrect ended time format."))?;
        Ok(LogRecord {
            task: text.to_owned(),
            started,
            ended,
        })
    }
}

pub struct LogFile(Vec<LogRecord>);

impl LogDir {
    pub fn new_file(root: &Path) -> Result<LogDir> {
        if !root.exists() {
            std::fs::create_dir_all(root)?;
        }
        Ok(LogDir(root.to_owned()))
    }

    pub fn create_today_file(&self) -> Result<()> {
        todo!()
    }

    pub fn get_today_log(&self) -> Result<Vec<LogRecord>> {
        todo!()
    }

    pub fn append_today_log(&self, log: &LogRecord) -> Result<()> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[serde_as]
    #[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
    pub struct CustomStatusTime {
        #[serde_as(as = "StatusTime")]
        time: NaiveDateTime,
    }

    // #[serde_as]
    // #[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
    // pub struct CustomLogTime {
    //     #[serde_as(as = "LogTime")]
    //     time: NaiveTime,
    // }

    #[test]
    fn test_serialize_status_time() {
        let s = r#"{"time": "2021-12-21T11:23:45"}"#;
        let custom_time: CustomStatusTime = serde_json::from_str(s).unwrap();
        assert_eq!(
            custom_time,
            CustomStatusTime {
                time: NaiveDate::from_ymd(2021, 12, 21).and_hms(11, 23, 45)
            }
        );
    }

    // #[test]
    // fn test_serialize_log_time() {
    //     let s = r#"{"time": "12:34"}"#;
    //     let desered: CustomLogTime = serde_json::from_str(s).unwrap();
    //     assert_eq!(
    //         desered,
    //         CustomLogTime {
    //             time: NaiveTime::from_hms(12, 34, 0)
    //         }
    //     );
    //
    //     let custom_time = CustomLogTime {
    //         time: NaiveTime::from_hms(1, 3, 5),
    //     };
    //     let text = serde_json::to_string(&custom_time).unwrap();
    //     assert_eq!(text, r#"{"time":"01:03"}"#.to_owned());
    // }
}
