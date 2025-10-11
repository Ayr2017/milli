use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct DeleteIndexRequest {
    pub uid: String,
}