use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct TestIndexDataQueryRequest {
    pub uid: String,
    pub data_source_id: u32,
    pub query: String,
}