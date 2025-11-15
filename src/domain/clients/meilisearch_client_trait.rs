use std::sync::Arc;
use meilisearch_sdk::client::Client;

pub trait MeilisearchClientTrait {
    fn new(meilisearch_client:Arc<Client>) -> Self;
    fn index(&self, index_name: &str) -> meilisearch_sdk::indexes::Index;

    async fn add_documents(&self, index_name: &str, documents: &[serde_json::Value]) -> Result<(), Box<dyn std::error::Error>> {
        let index = self.index(index_name);
        index.add_documents(documents, None).await?;
        Ok(())
    }

}