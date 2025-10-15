use dotenv::dotenv;
use std::env;

#[derive(Debug, Clone)]
pub struct ApplicationConfig {
    pub meilisearch_host: String,
    pub meilisearch_key: String,
    pub meilisearch_port: String,
    pub db_path: String,
    pub db_name: String,
    pub db_user: String,
    pub db_pass: String,
}

#[derive(Debug)]
pub struct ConfigError {
    pub(crate) message: String,
}

impl ApplicationConfig {
    pub async fn new() -> Result<Self, ConfigError> {
        dotenv().ok();
        Ok(Self {
            meilisearch_host: env::var("MEILI_HOST").map_err(|_| ConfigError {
                message: "MEILI_HOST not found in environment".to_string(),
            })?,
            meilisearch_port: env::var("MEILI_PORT").map_err(|_| ConfigError {
                message: "MEILI_PORT not found in environment".to_string(),
            })?,
            meilisearch_key: env::var("MEILI_MASTER_KEY").map_err(|_| ConfigError {
                message: "MEILI_MASTER_KEY not found in environment".to_string(),
            })?,
            db_path: env::var("DB_PATH").map_err(|_| ConfigError {
                message: "DB_PATH not found in environment".to_string(),
            })?,
            db_name: env::var("DB_NAME").map_err(|_| ConfigError {
                message: "DB_NAME not found in environment".to_string(),
            })?,
            db_user: env::var("DB_USER").map_err(|_| ConfigError {
                message: "DB_USER not found in environment".to_string(),
            })?,
            db_pass: env::var("DB_PASS").map_err(|_| ConfigError {
                message: "DB_PASS not found in environment".to_string(),
            })?,
        })
    }

    pub fn get_meilisearch_url(&self) -> String {
        format!("{}:{}", self.meilisearch_host, self.meilisearch_port)
    }
    pub fn get_meilisearch_key(&self) -> String {
        format!("{}", self.meilisearch_key,)
    }
    pub fn get_db_name(&self) -> String {
        format!("{}", self.db_name,)
    }
    pub fn get_db_user(&self) -> String {
        format!("{}", self.db_user,)
    }
    pub fn get_db_pass(&self) -> String {
        format!("{}", self.db_pass,)
    }
}