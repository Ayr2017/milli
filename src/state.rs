use std::sync::Arc;
use meilisearch_sdk::errors::Error;
use meilisearch_sdk::client::Client;
use crate::config::application::ApplicationConfig;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<ApplicationConfig>,
    pub meilisearch_client: Arc<Client>,
}

impl AppState {
    pub fn new(config: ApplicationConfig) -> Self {
        let meilisearch_client = Arc::new(
            Client::new(
                config.get_meilisearch_url(),
                Some(config.get_meilisearch_key()),
            ).expect("Failed to create Meilisearch client"),
        );

        Self {
            config: Arc::new(config),
            meilisearch_client,
        }
    }
}