use sqlx::Executor;
use crate::database::Database;
use crate::domain::repository::data_source_repository_trait::DataSourceRepositoryTrait;
use crate::requests::index_data_query::test_index_data_query_request::TestIndexDataQueryRequest;

pub struct TestIndexDataQueryUseCase <R:DataSourceRepositoryTrait>{
    repo: R,
}

impl <R:DataSourceRepositoryTrait> TestIndexDataQueryUseCase <R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
    
    pub fn execute(
        &self,
        payload: &TestIndexDataQueryRequest,
    ) -> Result<(), String> {
        let data_source_id = payload.data_source_id;
        let data_source = self.repo.get(data_source_id);
        println!("{:?}", payload);
        Ok(())
    }
}