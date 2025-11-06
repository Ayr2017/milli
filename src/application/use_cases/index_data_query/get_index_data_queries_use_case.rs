use crate::domain::data_source::entities::index_data_query::IndexDataQuery;
use crate::domain::repository::index_data_query_repository_trait::IndexDataQueryRepositoryTrait;
use crate::infrastructure::repositories::index_data_query_repository::IndexDataQueryRepository;
use crate::presentation::requests::index_data_query::get_index_data_query_request_dto::GetIndexDataQueryRequest;
use crate::requests::index_data_query::index_index_data_query_request::IndexIndexDataQueryRequest;

pub struct GetIndexDataQueriesUseCase <R: IndexDataQueryRepositoryTrait> {
    pub repo: R,
}

impl <R: IndexDataQueryRepositoryTrait>  GetIndexDataQueriesUseCase <R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
    
    pub async fn execute(
        &self, 
        _query: &IndexIndexDataQueryRequest
    ) -> Result<Vec<IndexDataQuery>, String> {
        match self.repo.all().await {
            Ok(index_data_queries) => Ok(index_data_queries),
            Err(_) => Err(String::from("Error getting index data queries")),
        }
    }
}