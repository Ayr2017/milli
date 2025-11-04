use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StoreIndexDataQueryRequest {
    pub data_source_id: u32,
    pub index_uid: String,
    pub query: String,
}