use crate::application::use_cases::index::dto::index_stat_dto::IndexStatsDto;
use std::collections::HashMap;
use serde::Serialize;
use time::OffsetDateTime;

#[derive(Serialize)]
pub struct IndexDataDto {
    pub uid: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub primary_key: String,
    pub stats: IndexStatsDto,
    pub searchable_attributes: Vec<String>,
    pub filterable_attributes: Vec<String>,
    pub sortable_attributes: Vec<String>,
    pub displayable_attributes: Vec<String>,
    pub ranking_rules: Vec<String>,
    pub stop_words: Vec<String>,
    pub synonyms: HashMap<String, Vec<String>>,
    pub distinct_attribute: Option<String>,
}

impl IndexDataDto {
    pub fn new(
        uid: String,
        created_at: OffsetDateTime,
        updated_at: OffsetDateTime,
        primary_key: String,
        stats: IndexStatsDto,
        searchable_attributes: Vec<String>,
        filterable_attributes: Vec<String>,
        sortable_attributes: Vec<String>,
        displayable_attributes: Vec<String>,
        ranking_rules: Vec<String>,
        stop_words: Vec<String>,
        synonyms: HashMap<String, Vec<String>>,
        distinct_attribute: Option<String>,
    ) -> Self {
        Self {
            uid,
            created_at,
            updated_at,
            primary_key,
            stats,
            searchable_attributes,
            filterable_attributes,
            sortable_attributes,
            displayable_attributes,
            ranking_rules,
            stop_words,
            synonyms,
            distinct_attribute,
        }
    }
}
