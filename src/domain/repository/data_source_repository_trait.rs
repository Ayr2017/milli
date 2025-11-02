use crate::database::Database;
use std::sync::Arc;
use crate::domain::data_source::entities::data_source::DataSource;

pub trait DataSourceRepositoryTrait {
    fn new(db:Database) -> Self;
    async fn get(&self, id: u32) -> Option<DataSource>;
    fn all(&self) -> Vec<Self> where Self: Sized;
    fn store(&self, data: Self) -> Self;
    fn update(&self, id: i32, data: Self) -> Option<Self> where Self: Sized;
    fn delete(&self, id: i32) -> Option<Self> where Self: Sized;
    fn delete_all(&self) -> Vec<Self> where Self: Sized;
    fn count(&self) -> i32;
    
}