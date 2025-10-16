use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct TestDataSourceRequest {
    pub id: u32
}