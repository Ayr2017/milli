use dotenv::dotenv;
use std::env;

#[derive(Debug)]
pub struct ApplicationConfig {
    pub meilisearch_host: String,
    pub meilisearch_key: String,
    pub meilisearch_port: String,
}

#[derive(Debug)]
pub struct ConfigError {
    message: String,
}

impl ApplicationConfig {
    pub fn new() -> Result<Self, ConfigError> {
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
        })
    }

    pub fn get_meilisearch_url(&self) -> String {
        format!("{}:{}", self.meilisearch_host, self.meilisearch_port)
    }
    pub fn get_meilisearch_key(&self) -> String {
        format!("{}", self.meilisearch_key,)
    }
}