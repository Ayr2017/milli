use crate::database::Database;
use crate::domain::data_source::entities::data_source::DataSource;
use crate::domain::data_source::entities::index_data_query::IndexDataQuery;
use crate::domain::repository::data_source_repository_trait::DataSourceRepositoryTrait;
use crate::presentation::requests::index_data_query::store_index_data_query_request::StoreIndexDataQueryRequest;
use colored::Colorize;
use sqlx::{Executor, Row};
use crate::requests::data_source::store_data_source_request::StoreDataSourceRequest;

pub struct DataSourceRepository {
    pub db: Database,
}

impl DataSourceRepositoryTrait for DataSourceRepository {
    fn new(db: Database) -> Self {
        DataSourceRepository { db }
    }

    async fn get(&self, id: u32) -> Option<DataSource>
    where
        Self: Sized,
    {
        let connection = self.db.get_pool_connection().await.unwrap();
        println!("get data_source by id: {}", id.to_string().blue());
        let result = sqlx::query("SELECT * FROM data_sources WHERE id = $1")
            .bind(id)
            .fetch_one(connection)
            .await;

        match &result {
            Ok(_) => println!("Query result: Ok"),
            Err(e) => println!("Query result: Err({:?})", e),
        }

        match result {
            Ok(row) => Some(DataSource {
                id,
                name: row.get("name"),
                host: row.get("host"),
                database: row.get("database"),
                username: row.get("username"),
                password: row.get("password"),
                port: row.get("port"),
                database_path: row.get("database_path"),
                database_name: row.get("database_name"),
                database_type: row.get("database_type"),
            }),
            Err(_) => None,
        }
    }

    fn all(&self) -> Vec<Self>
    where
        Self: Sized,
    {
        Vec::new()
    }

    async fn store(&self, data: StoreDataSourceRequest) -> Option<DataSource> {
        todo!()
    }


    fn update(&self, id: i32, data: Self) -> Option<Self>
    where
        Self: Sized,
    {
        Some(data)
    }

    fn delete(&self, id: i32) -> Option<Self>
    where
        Self: Sized,
    {
        None
    }

    fn delete_all(&self) -> Vec<Self>
    where
        Self: Sized,
    {
        Vec::new()
    }

    fn count(&self) -> i32 {
        0
    }
}
