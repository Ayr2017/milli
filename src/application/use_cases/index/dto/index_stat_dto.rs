use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct IndexStatsDto {
    pub number_of_documents: u64,
    pub is_indexing: bool,
}

impl IndexStatsDto {
    pub fn new(number_of_documents: u64, is_indexing: bool) -> Self {
        Self {
            number_of_documents,
            is_indexing,
        }
    }
}