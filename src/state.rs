use std::sync::Arc;
use meilisearch_sdk::errors::Error;
use meilisearch_sdk::client::Client;
use crate::config::application::ApplicationConfig;
use crate::database::Database;

#[derive(Clone, Debug)]
pub struct AppState {
    pub config: Arc<ApplicationConfig>,
    pub meilisearch_client: Arc<Client>,
    pub database: Option<Arc<Database>>,
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
            database: None,
        }
    }
    
    pub fn set_database(&mut self, database: Arc<Database>) -> &mut Self {
        self.database = Some(database);
        self
    }

}