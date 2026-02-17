use std::sync::Arc;
use meilisearch_sdk::client::Client;
use crate::config::application::ApplicationConfig;
use crate::database::Database;
use crate::queues::application::queue_service::JobService;
use crate::modules::queue::storage::repositories::job_repository::JobRepository as ModuleJobRepository;
use crate::queues::infrastructure::repositories::job_repository_adapter::{JobRepositoryAdapter, FailedJobRepositoryStub};

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<ApplicationConfig>,
    pub meilisearch_client: Arc<Client>,
    pub database: Arc<Database>,
    pub job_service: Arc<JobService>,
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

        let job_service = Self::get_job_service(database.clone()).await;

        Ok(Self {
            config: Arc::new(config),
            meilisearch_client,
            database: Arc::new(database),
            job_service,
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
        let job_service = Self::get_job_service(temp_db.clone()).await;


        Ok(Self {
            config: Arc::new(config),
            meilisearch_client,
            database: Arc::new(temp_db),
            job_service,
        })
    }
    
    pub async fn get_job_service(database: Database) -> Arc<JobService> {
        // Создаем репозитории и сервис для работы с очередями
        let module_job_repository = Arc::new(ModuleJobRepository::new(database.pool.clone()));
        let job_repository_adapter = Arc::new(JobRepositoryAdapter::new(module_job_repository));
        let failed_job_repository_stub = Arc::new(FailedJobRepositoryStub::new());
        let job_service = Arc::new(JobService::new(
        job_repository_adapter,
        failed_job_repository_stub,
        ));
        
        job_service
    }


}
