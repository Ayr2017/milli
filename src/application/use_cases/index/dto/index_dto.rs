use serde::Serialize;
use chrono::{DateTime, Utc};
use time::OffsetDateTime;

#[derive(Serialize, Debug, Clone)]
pub struct IndexDto {
    pub uid: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub primary_key: String,
}

impl IndexDto {
    pub fn new(
        uid: String,
        created_at: OffsetDateTime,
        updated_at: OffsetDateTime,
        primary_key: String,
    ) -> Self {
        Self {
            uid,
            created_at,
            updated_at,
            primary_key,
        }
    }
}