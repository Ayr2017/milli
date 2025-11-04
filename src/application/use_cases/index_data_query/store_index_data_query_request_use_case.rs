use std::sync::Arc;
use anyhow::Error;
use axum::extract::Query;
use crate::database::Database;
use crate::domain::repository::data_source_repository_trait::DataSourceRepositoryTrait;
use crate::domain::repository::index_data_query_repository_trait::IndexDataQueryRepositoryTrait;
use crate::presentation::requests::index_data_query::store_index_data_query_request::StoreIndexDataQueryRequest;

pub struct StoreIndexDataQueryRequestUseCase <R: IndexDataQueryRepositoryTrait> {
    repo:R,
}

impl <R: IndexDataQueryRepositoryTrait> StoreIndexDataQueryRequestUseCase <R> {
    pub async  fn new(repo: R) -> Self {
        Self { repo }
    }
    pub async fn execute(
        &self,
        payload: &StoreIndexDataQueryRequest,
    ) -> Result<String, Error> {
        let result = self.repo.store(payload).await;
        match result {
            Some(_) => Ok("Success".to_string()),
            None => Err(Error::msg("Failed to store index data query".to_string())),
        }
    }
}

