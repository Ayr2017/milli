use colored::Colorize;
use crate::domain::data_source::entities::data_source::DataSource;
use crate::domain::repository::data_source_repository_trait::DataSourceRepositoryTrait;
use crate::requests::index_data_query::test_index_data_query_request::TestIndexDataQueryRequest;
use serde_json::Value;
use sqlx::postgres::PgPool;
use sqlx::postgres::PgRow;
use sqlx::{Column, Executor, Row};

pub struct TestIndexDataQueryUseCase<R: DataSourceRepositoryTrait> {
    repo: R,
}

impl<R: DataSourceRepositoryTrait> TestIndexDataQueryUseCase<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, payload: &TestIndexDataQueryRequest) -> Result<Value, String> {
        let data_source_id = payload.data_source_id;
        let data_source = self.repo.get(data_source_id).await.ok_or("Data source not found".to_string())?;
        let result = self.execute_query(&data_source, &payload.query).await;
        println!("{:?}", result);
        println!("{:?}", payload);
        return result;
    }

    async fn execute_query(
        &self,
        data_source: &DataSource,
        query: &str,
    ) -> Result<serde_json::Value, String> {
        let query_without_semicolon = query.strip_suffix(";").unwrap_or(query);
        let limited_query = format!("{} LIMIT 1", query_without_semicolon);
        let connection_string = format!(
            "postgresql://{}:{}@{}:{}/{}",
            data_source.username,
            data_source.password,
            data_source.host,
            data_source.port,
            data_source.database_name
        );
        
        println!("Connection string: {}", connection_string.color("blue"));

        let pool = PgPool::connect(&connection_string)
            .await
            .map_err(|e| format!("Database connection error: {}", e))?;

        let row = sqlx::query(&limited_query)
            .fetch_optional(&pool)
            .await
            .map_err(|e| format!("Query execution error: {}", e))?;

        // Закрываем подключение
        pool.close().await;

        // Простое преобразование в JSON
        Ok(match row {
            Some(row) => Self::row_to_json(row),
            None => serde_json::Value::Null,
        })
    }

    fn row_to_json(row: PgRow) -> serde_json::Value {
        let mut json_map = serde_json::Map::new();

        for column in row.columns() {
            let column_name = column.name();
            let value = match row.try_get::<Option<String>, _>(column_name) {
                Ok(Some(val)) => serde_json::Value::String(val),
                Ok(None) => serde_json::Value::Null,
                Err(_) => serde_json::Value::Null,
            };
            json_map.insert(column_name.to_string(), value);
        }

        serde_json::Value::Object(json_map)
    }
}
