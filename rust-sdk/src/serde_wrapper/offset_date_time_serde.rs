use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::str::FromStr;
use time::OffsetDateTime;

#[derive(Clone, Debug)]
pub struct OffsetDateTimeSerdeAsTimeStampStr;

impl OffsetDateTimeSerdeAsTimeStampStr {
    pub fn serialize<S>(val: &OffsetDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let ts_str = format!("{}", val.unix_timestamp());
        String::serialize(&ts_str, serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let ts_str = String::deserialize(deserializer)?;
        let ts = i64::from_str(&ts_str)
            .map_err(|e| de::Error::custom(format!("string to i64 error: {}", e)))?;
        OffsetDateTime::from_unix_timestamp(ts).map_err(|e| {
            de::Error::custom(format!("timestamp {} to OffsetDateTime error: {}", ts, e))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_offset_time_serialize() {
        #[derive(Serialize, Deserialize)]
        struct TimeSerdeTS {
            #[serde(with = "OffsetDateTimeSerdeAsTimeStampStr")]
            t: OffsetDateTime,
        }

        let expect_ts = 1_546_300_800;
        let value = TimeSerdeTS {
            t: OffsetDateTime::from_unix_timestamp(expect_ts).unwrap(),
        };

        assert_eq!(
            serde_json::to_string(&value).unwrap(),
            format!(r#"{{"t":"{}"}}"#, expect_ts)
        );
    }

    #[test]
    fn test_offset_time_deserialize() {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct TimeSerdeTS {
            #[serde(with = "OffsetDateTimeSerdeAsTimeStampStr")]
            t: OffsetDateTime,
        }

        const EXPECT_TS: i64 = 1_546_300_800;
        let json_str = format!(r#"{{"t":"{}"}}"#, EXPECT_TS);

        let value = serde_json::from_str::<TimeSerdeTS>(&json_str).unwrap();
        assert_eq!(
            value,
            TimeSerdeTS {
                t: OffsetDateTime::from_unix_timestamp(EXPECT_TS).unwrap()
            }
        )
    }
}
