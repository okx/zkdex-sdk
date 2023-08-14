use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

pub struct StdDurationSerdeAsSecondsStr;

impl StdDurationSerdeAsSecondsStr {
    pub fn serialize<S>(val: &std::time::Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        String::serialize(&val.as_secs().to_string(), serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<std::time::Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        let seconds_str = String::deserialize(deserializer)?;
        let seconds = seconds_str
            .parse()
            .map_err(|e| de::Error::custom(format!("parse {} to u64 error: {}", seconds_str, e)))?;

        Ok(std::time::Duration::from_secs(seconds))
    }
}

pub struct StdDurationSerdeAsSecondsU64;

impl StdDurationSerdeAsSecondsU64 {
    pub fn serialize<S>(val: &std::time::Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        u64::serialize(&val.as_secs(), serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<std::time::Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        let seconds = u64::deserialize(deserializer)?;
        Ok(std::time::Duration::from_secs(seconds))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_std_duration_str_serialize() {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct StdDurationSerde {
            #[serde(with = "StdDurationSerdeAsSecondsStr")]
            d: std::time::Duration,
        }

        let expect_duration = std::time::Duration::from_millis(300_8000000);
        let value = StdDurationSerde { d: expect_duration };

        assert_eq!(
            serde_json::to_string(&value).unwrap(),
            format!(r#"{{"d":"{}"}}"#, expect_duration.as_secs())
        );
    }

    #[test]
    fn test_std_duration_str_deserialize() {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct StdDurationSerde {
            #[serde(with = "StdDurationSerdeAsSecondsStr")]
            d: std::time::Duration,
        }

        const EXPECT_SECS: u64 = 300_8000;
        let json_str = format!(r#"{{"d":"{}"}}"#, EXPECT_SECS);

        assert_eq!(
            serde_json::from_str::<StdDurationSerde>(&json_str).unwrap(),
            StdDurationSerde {
                d: std::time::Duration::from_secs(EXPECT_SECS)
            }
        );
    }

    #[test]
    fn test_std_duration_u64_serialize() {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct StdDurationSerde {
            #[serde(with = "StdDurationSerdeAsSecondsU64")]
            d: std::time::Duration,
        }

        let expect_duration = std::time::Duration::from_millis(300_8000000);
        let value = StdDurationSerde { d: expect_duration };

        assert_eq!(
            serde_json::to_string(&value).unwrap(),
            format!(r#"{{"d":{}}}"#, expect_duration.as_secs())
        );
    }

    #[test]
    fn test_std_duration_u64_deserialize() {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct StdDurationSerde {
            #[serde(with = "StdDurationSerdeAsSecondsU64")]
            d: std::time::Duration,
        }

        const EXPECT_SECS: u64 = 300_8000;
        let json_str = format!(r#"{{"d":{}}}"#, EXPECT_SECS);

        assert_eq!(
            serde_json::from_str::<StdDurationSerde>(&json_str).unwrap(),
            StdDurationSerde {
                d: std::time::Duration::from_secs(EXPECT_SECS)
            }
        );
    }
}
