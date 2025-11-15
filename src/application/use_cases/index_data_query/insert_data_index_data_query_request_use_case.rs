use anyhow::Error;
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
}

impl <R: IndexDataQueryRepositoryTrait, R2: DataSourceRepositoryTrait> InsertDataIndexDataQueryRequestUseCase <R, R2> {
    pub async  fn new(index_data_query_repository: R, data_source_repository: R2) -> Self {
        Self { 
            index_data_query_repository,
            data_source_repository,
        }
    }
    pub async fn execute(
        &self,
        payload: &InsertDataIndexDataQueryRequest,
    ) -> Result<String, Error> {
        let query_executor = QueryExecutor::new();
        let index_data_query = Self::get_index_data_query(&self, payload).await;
        let data_source_id = index_data_query.data_source_id;
        let index_uid = index_data_query.index_uid;
        let query = index_data_query.query;
        let limit = 1;

        let data_source = self.data_source_repository.get(data_source_id).await.unwrap();
        println!("Data source: {:?}", &data_source);

        let result = query_executor.execute_query(&data_source, &query, limit).await;
        println!("Result: {:?}", &result);

        let document = match result {
            Ok(document) => {
                // Преобразуем в чистый JSON
                serde_json::to_value(document).map_err(|e| anyhow::anyhow!("Serialization error: {}", e))?
            },
            Err(e) => {
                println!("Error: {:?}", e);
                return Err(anyhow::anyhow!("Error: {:?}", e));
            },
        };

        println!("Document: {}", &document); // Теперь выведет чистый JSON
        
        

        // этот json отправить на добавление в index
        // Temporary stub implementation
        Ok("Success - Data insertion request processed".to_string())
    }
    
    async fn get_index_data_query(&self, request: &InsertDataIndexDataQueryRequest) -> IndexDataQuery {
        self.index_data_query_repository.get(request.id).await.unwrap()
    }
}

