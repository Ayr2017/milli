use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize, Debug, Clone)]
pub struct IndexSettingsDto {
    pub searchable_attributes: Vec<String>,
    pub filterable_attributes: Vec<String>,
    pub sortable_attributes: Vec<String>,
    pub displayable_attributes: Vec<String>,
    pub ranking_rules: Vec<String>,
    pub stop_words: Vec<String>,
    pub synonyms: HashMap<String, Vec<String>>,
    pub distinct_attribute: Option<String>,
}

impl IndexSettingsDto {
    pub fn new(
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