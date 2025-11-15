use anyhow::Error;
use chrono::Utc;
use meilisearch_sdk::client::Client;
use meilisearch_sdk::tasks::Task;
use serde_json::json;
use crate::domain::data_source::entities::data_source::DataSource;
use crate::domain::data_source::entities::index_data_query::IndexDataQuery;
use crate::domain::repository::index_data_query_repository_trait::IndexDataQueryRepositoryTrait;
use crate::presentation::requests::index_data_query::store_index_data_query_request::StoreIndexDataQueryRequest;
use crate::requests::index_data_query::insert_data_index_data_query_request::InsertDataIndexDataQueryRequest;
use crate::domain::data_source::services::query_executor::QueryExecutor;
use crate::domain::repository::data_source_repository_trait::DataSourceRepositoryTrait;

pub struct InsertDataIndexDataQueryRequestUseCase <R: IndexDataQueryRepositoryTrait, R2: DataSourceRepositoryTrait> {
    index_data_query_repository:R,
    data_source_repository:R2,
    meilisearch_client: Client,
}

impl <R: IndexDataQueryRepositoryTrait, R2: DataSourceRepositoryTrait> InsertDataIndexDataQueryRequestUseCase <R, R2> {
    pub async  fn new(index_data_query_repository: R, data_source_repository: R2, meilisearch_client: Client) -> Self {
        Self { 
            index_data_query_repository,
            data_source_repository,
            meilisearch_client,
        }
    }
    pub async fn execute(
        &self,
        payload: &InsertDataIndexDataQueryRequest,
    ) -> Result<String, Error> {
        
        // нужно работать с батчами
        // измерить нужное количество батчей и через цикл получать из бд и отправлять
        
        let query_executor = QueryExecutor::new();
        let index_data_query = Self::get_index_data_query(&self, payload).await;
        let data_source_id = index_data_query.data_source_id;
        let index_uid = index_data_query.index_uid;
        let query = index_data_query.query;
        let limit = 3_000;

        let data_source = self.data_source_repository.get(data_source_id).await.unwrap();

        let result = query_executor.execute_query(&data_source, &query, limit).await;

        let document = match result {
            Ok(document) => {
                // Преобразуем в чистый JSON
                document
            },
            Err(e) => {
                println!("Error: {:?}", e);
                return Err(anyhow::anyhow!("Error: {:?}", e));
            },
        };

        let meiliserach_insert_result = match self.meilisearch_client
            .index(&index_uid.to_string())
            .add_documents(&document, Some("id"))
            .await
        {
            Ok(task_info) => {
                // Ждем завершения задачи
                let final_task = self.meilisearch_client
                    .wait_for_task(task_info, None, None)
                    .await?;

                println!("Final task result: {:#?}", final_task);

                match final_task {
                    Task::Succeeded { content, .. } => {
                        println!("✅ Documents added successfully!");
                        println!("Task details: {:?}", content);
                        Ok("Success - Data inserted successfully".to_string())
                    },
                    Task::Failed { content, .. } => {
                        println!("❌ Task failed: {:?}", content);
                        Err(anyhow::anyhow!("Meilisearch task failed: {:?}", content))
                    },
                    _ => {
                        println!("⚠️ Task still in progress");
                        Ok("Success - Data insertion in progress".to_string())
                    },
                }
            },
            Err(e) => {
                println!("Error enqueueing task: {:?}", e);
                Err(anyhow::anyhow!("Error: {:?}", e))
            },
        };
        


        // этот json отправить на добавление в index
        // Temporary stub implementation
        Ok((Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()  ))
    }
    
    async fn get_index_data_query(&self, request: &InsertDataIndexDataQueryRequest) -> IndexDataQuery {
        self.index_data_query_repository.get(request.id).await.unwrap()
    }
}

