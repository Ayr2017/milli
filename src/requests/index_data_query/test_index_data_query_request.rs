use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct TestIndexDataQueryRequest {
    pub uid: String,
    pub query: String,
}