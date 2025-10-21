use std::sync::Arc;
use meilisearch_sdk::client::Client;
use crate::config::application::ApplicationConfig;
use crate::database::Database;

#[derive(Clone, Debug)]
pub struct AppState {
    pub config: Arc<ApplicationConfig>,
    pub meilisearch_client: Arc<Client>,
    pub database: Arc<Database>,
}

impl AppState {
    pub async fn new(
        config: ApplicationConfig,
        database: Database,
    ) -> Result< Self, anyhow::Error> {
        let meilisearch_client = Arc::new(
            Client::new(
                config.get_meilisearch_url(),
                Some(config.get_meilisearch_key()),
            ).expect("Failed to create Meilisearch client"),
        );

        Ok(Self {
            config: Arc::new(config),
            meilisearch_client,
            database: Arc::new(database),
        })

    }

    /// Создает состояние приложения только с конфигурацией (для тестирования)
    #[cfg(test)]
    pub async fn new_without_database(config: ApplicationConfig) -> Result<Self, anyhow::Error> {
        let meilisearch_client = Arc::new(
            Client::new(
                config.get_meilisearch_url(),
                Some(config.get_meilisearch_key()),
            ).map_err(|e| anyhow::anyhow!("Failed to create Meilisearch client: {}", e))?,
        );

        // Создаем временную базу данных в памяти для тестов
        let temp_db = Database::new(":memory:").await?;

        Ok(Self {
            config: Arc::new(config),
            meilisearch_client,
            database: Arc::new(temp_db),
        })
    }


}
