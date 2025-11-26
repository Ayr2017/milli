use anyhow::Error;
use crate::application::use_cases::index::dto::index_data_dto::IndexDataDto;
use crate::application::use_cases::index::dto::index_dto::IndexDto;
use crate::application::use_cases::index::dto::index_settings_dto::IndexSettingsDto;
use crate::application::use_cases::index::dto::index_stat_dto::IndexStatsDto;
use meilisearch_sdk::client::Client;
use meilisearch_sdk::indexes::IndexStats;
use meilisearch_sdk::settings::Settings;

pub struct ShowIndexUseCase {
    client: Client,
    uid: String,
}

impl ShowIndexUseCase {
    pub fn new(client: Client, uid: String) -> Self {
        Self { client, uid }
    }

    pub async fn execute(&self) -> Result<IndexDataDto, Error> {
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

        let index_dto = self.prepare_index_dto(index).await;
        let index_stats_dto = self.prepare_index_stats_dto(index_stats).await;
        let index_settings_dto = self.prepare_index_settings(index_settings).await;
        let index_data_dto = IndexDataDto {
            uid: index_dto.uid,
            created_at: index_dto.created_at,
            updated_at: index_dto.updated_at,
            primary_key: index_dto.primary_key,
            stats: index_stats_dto,
            searchable_attributes: index_settings_dto.searchable_attributes,
            filterable_attributes: index_settings_dto.filterable_attributes,
            sortable_attributes: index_settings_dto.sortable_attributes,
            displayable_attributes: index_settings_dto.displayable_attributes,
            ranking_rules: index_settings_dto.ranking_rules,
            stop_words: index_settings_dto.stop_words,
            synonyms: index_settings_dto.synonyms,
            distinct_attribute: index_settings_dto.distinct_attribute,
        };

        Ok(index_data_dto)
    }
    /// This function prepares the index stats dto
    async fn prepare_index_stats_dto(&self,index_stats: IndexStats) -> IndexStatsDto {
        IndexStatsDto {
            number_of_documents: index_stats.number_of_documents as u64,
            is_indexing: index_stats.is_indexing,
        }
    }

    /// This function prepares the index dto
    async fn prepare_index_dto(&self,index: meilisearch_sdk::indexes::Index) -> IndexDto {
        IndexDto {
            uid: index.uid,
            created_at: index.created_at.unwrap(),
            updated_at: index.updated_at.unwrap(),
            primary_key: index.primary_key.unwrap_or("".to_string()),
        }
    }
    /// This function prepares the index settings dto
    async fn prepare_index_settings(&self,index_settings: Settings) -> IndexSettingsDto {
        IndexSettingsDto {
            searchable_attributes: index_settings.searchable_attributes.unwrap(),
            filterable_attributes: index_settings.filterable_attributes.unwrap(),
            sortable_attributes: index_settings.sortable_attributes.unwrap(),
            displayable_attributes: index_settings.displayed_attributes.unwrap(),
            ranking_rules: index_settings.ranking_rules.unwrap(),
            stop_words: index_settings.stop_words.unwrap(),
            synonyms: index_settings.synonyms.unwrap(),
            distinct_attribute: Some("".to_string()),
        }
    }
}


