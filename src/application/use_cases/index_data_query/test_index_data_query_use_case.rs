use axum::extract::Query;
use crate::requests::index_data_query::test_index_data_query_request::TestIndexDataQueryRequest;

pub struct TestIndexDataQueryUseCase{
    
}

impl TestIndexDataQueryUseCase {
    pub fn new() -> Self {
        Self {}
    }
    
    pub fn execute(
        &self,
        Query(params): Query<TestIndexDataQueryRequest>,
    ) -> Result<(), String> {
        Ok(())
    }
}