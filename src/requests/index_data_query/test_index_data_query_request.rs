use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TestIndexDataQueryRequest {
    pub uid: String,
    pub data_source_id: u32,
    pub query: String,
}

impl TestIndexDataQueryRequest {
    pub fn serialise(&self)-> Result<String, serde_json::Error>  {
        serde_json::to_string(self)
    }
}