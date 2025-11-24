use std::sync::Arc;
use axum::extract::Path;
use meilisearch_sdk::client::Client;
use meilisearch_sdk::indexes::IndexStats;
use crate::application::use_cases::index::dto::index_dto::IndexDto;
use crate::application::use_cases::index::dto::index_stat_dto::IndexStatsDto;

pub struct ShowIndexUseCase {
    client: Client,
    uid: Path<String>,
}

impl ShowIndexUseCase {
    pub fn new(
        client: Client,
        uid: Path<String>,
    ) -> Self {
        Self {
            client,
            uid
        }
    }
    
    pub async fn execute(&self) -> String {
        let uid = self.uid.to_string();
        let index = match self.client.get_index(uid).await {
            Ok(index) => index,
            Err(e) => panic!("Failed to get index stats: {}", e),
        };
        
        let index_stats = match index.get_stats().await {
            Ok(stats) => stats,
            Err(e) => panic!("Failed to get index stats: {}", e),
        };
        
        let index_settings = match index.get_settings().await {
            Ok(settings) => settings,
            Err(e) => panic!("Failed to get index stats: {}", e),
        };
        
        let index_dto = prepare_index_dto(index).await;
        let index_stats_dto = prepare_index_stats_dto(index_stats).await;


        "Hello World".to_string()
    }
}

async fn prepare_index_stats_dto(index_stats: IndexStats) -> IndexStatsDto {
    IndexStatsDto{
        number_of_documents: index_stats.number_of_documents as u64,
        is_indexing: index_stats.is_indexing,
    }
}

async fn prepare_index_dto(index: meilisearch_sdk::indexes::Index) -> IndexDto {
    IndexDto{
        uid: index.uid,
        created_at: index.created_at.unwrap(),
        updated_at: index.updated_at.unwrap(),
        primary_key: index.primary_key.unwrap_or("".to_string()),
    }
}

