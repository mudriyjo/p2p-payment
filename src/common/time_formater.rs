use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serializer};

pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let formatted = date.to_rfc3339();
    serializer.serialize_str(&formatted)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    DateTime::parse_from_rfc3339(&s)
        .map(|dt| dt.with_timezone(&Utc))
        .map_err(serde::de::Error::custom)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    struct TestStruct {
        #[serde(with = "crate::common::time_formater")]
        timestamp: DateTime<Utc>,
    }

    #[test]
    fn test_serialize_deserialize() {
        let now = Utc::now();
        let test = TestStruct { timestamp: now };

        let json = serde_json::to_string(&test).unwrap();
        assert!(json.contains(&now.to_rfc3339()));

        let deserialized: TestStruct = serde_json::from_str(&json).unwrap();
        
        assert_eq!(
            deserialized.timestamp.timestamp(),
            now.timestamp()
        );
    }
}