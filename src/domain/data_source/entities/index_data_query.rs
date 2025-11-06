use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct IndexDataQuery {
    pub id: u32,
    pub data_source_id: u32,
    pub index_uid: String,
    pub query: String,
}