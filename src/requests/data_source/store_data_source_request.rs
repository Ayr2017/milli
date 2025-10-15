use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct StoreDataSourceRequest {
    pub name: String,
    pub host: String,
    pub database: String,
    pub username: String,
    pub password: String,
    pub port: u16,
    pub database_path: String,
    pub database_name: String,
    pub database_type: String,
    pub created_at: String,
    pub updated_at: String,
}