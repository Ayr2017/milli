use crate::database::Database;
use crate::domain::data_source::entities::index_data_query::IndexDataQuery;
use crate::domain::repository::data_source_repository_trait::DataSourceRepositoryTrait;
use crate::domain::repository::index_data_query_repository_trait::IndexDataQueryRepositoryTrait;
use crate::infrastructure::repositories::data_source_repository::DataSourceRepository;
use crate::presentation::requests::index_data_query::store_index_data_query_request::StoreIndexDataQueryRequest;
use serde_json::json;
use sqlx::sqlite::SqliteRow;
use sqlx::{Error, Row};

pub struct IndexDataQueryRepository {
    db: Database,
}

impl IndexDataQueryRepositoryTrait for IndexDataQueryRepository {
    fn new(db: Database) -> Self {
        Self { db }
    }

    async fn get(&self, id: u32) -> Option<IndexDataQuery> {
        todo!()
    }

    /**
     * Get all index data queries
     */
    async fn all(&self) -> Result<Vec<IndexDataQuery>, anyhow::Error> {
        let connection = self.db.get_pool_connection().await.unwrap();
        let result = sqlx::query(r#"SELECT * FROM index_data_queries"#)
            .fetch_all(connection)
            .await;

        match result {
            Ok(rows) => {
                let mut index_data_queries = Vec::new();
                for row in rows {
                    index_data_queries.push(IndexDataQuery {
                        id: row.get("id"),
                        data_source_id: row.get("data_source_id"),
                        index_uid: row.get("index_uid"),
                        query: row.get("query"),
                    });
                }
                Ok(index_data_queries)
            }
            Err(_) => Err(
                anyhow::Error::msg(
                    String::from("Error getting all index data queries")
                )
            ),
        }
    }

    async fn store(&self, data: &StoreIndexDataQueryRequest) -> Option<IndexDataQuery> {
        let data_source = data.data_source_id;
        let index_uid = data.index_uid.clone();
        let query = data.query.clone();

        let connection = self.db.get_pool_connection().await.unwrap();
        println!("Got connection successfully");

        let result = sqlx::query(r#"INSERT INTO index_data_queries ("data_source_id","index_uid","query") VALUES ($1,$2,$3) RETURNING *"#)
            .bind(data_source)
            .bind(index_uid)
            .bind(query)
            .fetch_one(connection)
            .await;

        match result {
            Ok(row) => Some(IndexDataQuery {
                id: row.get("id"),
                data_source_id: row.get("data_source_id"),
                index_uid: row.get("index_uid"),
                query: row.get("query"),
            }),
            Err(e) => {
                println!("Query result: Err({:?})", e);
                None
            }
        }
    }

    fn update(&self, id: i32, data: Self) -> Option<Self>
    where
        Self: Sized,
    {
        todo!()
    }

    fn delete(&self, id: i32) -> Option<Self>
    where
        Self: Sized,
    {
        todo!()
    }

    fn delete_all(&self) -> Vec<Self>
    where
        Self: Sized,
    {
        todo!()
    }

    fn count(&self) -> i32 {
        todo!()
    }
}
