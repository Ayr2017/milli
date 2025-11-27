use std::collections::HashMap;
use crate::database::Database;
use crate::domain::data_source::entities::index_data_query::IndexDataQuery;
use crate::presentation::requests::index_data_query::store_index_data_query_request::StoreIndexDataQueryRequest;

pub trait IndexDataQueryRepositoryTrait {
    fn new(db:Database) -> Self;
    async fn get(&self, id: u32) -> Option<IndexDataQuery>;
    async fn all(&self, filter:Option<HashMap<String,String>>) -> Result<Vec<IndexDataQuery>, anyhow::Error>;
    async fn store(&self, data: &StoreIndexDataQueryRequest) -> Option<IndexDataQuery>;
    fn update(&self, id: i32, data: Self) -> Option<Self> where Self: Sized;
    fn delete(&self, id: i32) -> Option<Self> where Self: Sized;
    fn delete_all(&self) -> Vec<Self> where Self: Sized;
    fn count(&self) -> i32;
    
}