use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct StoreIndexRequest {
    pub name: String,
    pub pkey: Option<String>,
}