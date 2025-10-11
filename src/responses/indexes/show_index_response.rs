// ShowIndexResponse.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ShowIndexResponse {
    pub uid: String,
    #[serde(with = "datetime_format")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "datetime_format")]
    pub updated_at: DateTime<Utc>,
    pub primary_key: Option<String>,
    pub stats: IndexStats,
    pub searchable_attributes: Option<Value>,
    pub filterable_attributes: Option<Value>,
    pub sortable_attributes: Option<Value>,
    pub displayable_attributes: Option<Value>,
    pub ranking_rules: Option<Value>,
    pub stop_words: Option<Value>,
    pub synonyms: Option<Value>,
    pub distinct_attribute: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IndexStats {
    pub number_of_documents: u64,
    pub is_indexing: bool,
}

mod datetime_format {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%Y-%m-%d %H:%M:%S";

    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, FORMAT)
            .map_err(serde::de::Error::custom)
    }
}

impl ShowIndexResponse {
    pub fn from_json_value(value: &Value) -> Result<Self, serde_json::Error> {
        serde_json::from_value(value.clone())
    }

    pub fn to_json_value(&self) -> Result<Value, serde_json::Error> {
        serde_json::to_value(self)
    }
}