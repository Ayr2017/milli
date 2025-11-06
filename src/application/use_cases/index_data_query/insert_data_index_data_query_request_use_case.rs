use anyhow::Error;
use crate::domain::data_source::entities::index_data_query::IndexDataQuery;
use crate::domain::repository::index_data_query_repository_trait::IndexDataQueryRepositoryTrait;
use crate::presentation::requests::index_data_query::store_index_data_query_request::StoreIndexDataQueryRequest;
use crate::requests::index_data_query::insert_data_index_data_query_request::InsertDataIndexDataQueryRequest;

pub struct InsertDataIndexDataQueryRequestUseCase <R: IndexDataQueryRepositoryTrait> {
    repo:R,
}

impl <R: IndexDataQueryRepositoryTrait> InsertDataIndexDataQueryRequestUseCase <R> {
    pub async  fn new(repo: R) -> Self {
        Self { repo }
    }
    pub async fn execute(
        &self,
        payload: &InsertDataIndexDataQueryRequest,
    ) -> Result<String, Error> {
        let index_data_query = Self::get_index_data_query(&self, payload).await;
        // получить data_source по index_data_query.data_source_id
        // получить индекс по data_source.index_uid из state.meilisearch_client; client.get_index(uid).await.unwrap();
        // из этого создать подключение (queryDB) к бд по параметрам из data_source
        // выполнить запрос в queryDB с лимитом 1 (для теста)
        // полученный ответ превратить в json и вывести в консоль
        // этот json отправить на добавление в index
        // Temporary stub implementation
        Ok("Success - Data insertion request processed".to_string())
    }
    
    async fn get_index_data_query(&self, request: &InsertDataIndexDataQueryRequest) -> IndexDataQuery {
        self.repo.get(request.id).await.unwrap()
    }
}

