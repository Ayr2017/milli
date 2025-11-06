use serde::{Deserialize, Serialize};
use crate::requests::index_data_query::test_index_data_query_request::TestIndexDataQueryRequest;

#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct InsertDataIndexDataQueryRequest{
    pub id: u32,
}

impl InsertDataIndexDataQueryRequest {
    pub fn serialise(&self)-> Result<String, serde_json::Error>  {
        serde_json::to_string(self)
    }
}