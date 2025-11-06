use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetIndexDataQueryRequest {
    pub page: u32,
    pub limit: u32,
}

impl GetIndexDataQueryRequest {
    pub fn serialise(&self)-> Result<String, serde_json::Error>  {
        serde_json::to_string(self)
    }
}