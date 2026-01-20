use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer};
use std::fmt;

/// 包装类型：可以将字符串或数字转换为 i64
#[derive(Debug)]
struct I64OrString(i64);

impl<'de> Deserialize<'de> for I64OrString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct I64OrStringVisitor;

        impl<'de> Visitor<'de> for I64OrStringVisitor {
            type Value = I64OrString;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string or an i64")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                value.parse().map(I64OrString).map_err(de::Error::custom)
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(I64OrString(value))
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if value > i64::MAX as u64 {
                    Err(de::Error::custom("u64 overflow"))
                } else {
                    Ok(I64OrString(value as i64))
                }
            }
        }

        deserializer.deserialize_any(I64OrStringVisitor)
    }
}

/// 包装类型：可以将字符串或数字转换为 String
#[derive(Debug)]
struct StringOrNumber(String);

impl<'de> Deserialize<'de> for StringOrNumber {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct StringOrNumberVisitor;

        impl<'de> Visitor<'de> for StringOrNumberVisitor {
            type Value = StringOrNumber;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string or a number")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(StringOrNumber(value.to_string()))
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(StringOrNumber(value.to_string()))
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(StringOrNumber(value.to_string()))
            }

            fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(StringOrNumber(value.to_string()))
            }
        }

        deserializer.deserialize_any(StringOrNumberVisitor)
    }
}

#[derive(Debug)]
pub struct Flink {
    pub jid: String,
    pub name: String,
    pub state: String,
    pub start_time: Option<i64>,
    pub end_time: Option<String>,
    pub duration: Option<String>,
    pub last_modification: Option<String>,
    pub tasks: TaskInfo,
}

#[derive(Deserialize, Debug)]
pub struct TaskInfo {
    pub total: i32,
    pub created: i32,
    pub scheduled: i32,
    pub deploying: i32,
    pub running: i32,
    pub finished: i32,
    pub canceling: i32,
    pub canceled: i32,
    pub failed: i32,
    pub reconciling: i32,
    pub initializing: i32,
}

impl<'de> Deserialize<'de> for Flink {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Jid,
            Name,
            State,
            #[serde(rename = "start-time")]
            StartTime,
            #[serde(rename = "end-time")]
            EndTime,
            Duration,
            #[serde(rename = "last-modification")]
            LastModification,
            Tasks,
        }

        struct FlinkVisitor;

        impl<'de> Visitor<'de> for FlinkVisitor {
            type Value = Flink;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Flink")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Flink, V::Error>
            where
                V: de::MapAccess<'de>,
            {
                let mut jid = None;
                let mut name = None;
                let mut state = None;
                let mut start_time = None;
                let mut end_time = None;
                let mut duration = None;
                let mut last_modification = None;
                let mut tasks = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Jid => {
                            if jid.is_some() {
                                return Err(de::Error::duplicate_field("jid"));
                            }
                            jid = Some(map.next_value()?);
                        }
                        Field::Name => {
                            if name.is_some() {
                                return Err(de::Error::duplicate_field("name"));
                            }
                            name = Some(map.next_value()?);
                        }
                        Field::State => {
                            if state.is_some() {
                                return Err(de::Error::duplicate_field("state"));
                            }
                            state = Some(map.next_value()?);
                        }
                        Field::StartTime => {
                            if start_time.is_some() {
                                return Err(de::Error::duplicate_field("start_time"));
                            }
                            let value: Option<I64OrString> = map.next_value()?;
                            start_time = value.map(|I64OrString(v)| v);
                        }
                        Field::EndTime => {
                            if end_time.is_some() {
                                return Err(de::Error::duplicate_field("end_time"));
                            }
                            let value: Option<StringOrNumber> = map.next_value()?;
                            end_time = value.map(|StringOrNumber(v)| v);
                        }
                        Field::Duration => {
                            if duration.is_some() {
                                return Err(de::Error::duplicate_field("duration"));
                            }
                            let value: Option<StringOrNumber> = map.next_value()?;
                            duration = value.map(|StringOrNumber(v)| v);
                        }
                        Field::LastModification => {
                            if last_modification.is_some() {
                                return Err(de::Error::duplicate_field("last_modification"));
                            }
                            let value: Option<StringOrNumber> = map.next_value()?;
                            last_modification = value.map(|StringOrNumber(v)| v);
                        }
                        Field::Tasks => {
                            if tasks.is_some() {
                                return Err(de::Error::duplicate_field("tasks"));
                            }
                            tasks = Some(map.next_value()?);
                        }
                    }
                }

                let jid = jid.ok_or_else(|| de::Error::missing_field("jid"))?;
                let name = name.ok_or_else(|| de::Error::missing_field("name"))?;
                let state = state.ok_or_else(|| de::Error::missing_field("state"))?;
                let tasks = tasks.ok_or_else(|| de::Error::missing_field("tasks"))?;

                Ok(Flink {
                    jid,
                    name,
                    state,
                    start_time,
                    end_time,
                    duration,
                    last_modification,
                    tasks,
                })
            }
        }

        const FIELDS: &[&str] = &[
            "jid",
            "name",
            "state",
            "start_time",
            "end_time",
            "duration",
            "last_modification",
            "tasks",
        ];
        deserializer.deserialize_struct("Flink", FIELDS, FlinkVisitor)
    }
}
