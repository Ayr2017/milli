use anyhow::Error;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use colored::Colorize;
use sqlx::{Pool, Sqlite, query_as};
use serde_json::json;
use crate::models::data_source::DataSource;
use crate::requests::data_source::test_data_source_request::TestDataSourceRequest;
use crate::state::AppState;

pub struct DataSourceService {
    pub pool: Pool<Sqlite>,
}

impl DataSourceService {
    pub fn new(pool: Pool<Sqlite>) -> DataSourceService {
        DataSourceService {
            pool,
        }
    }

    pub async fn test_data_source(
        &self,
        payload: TestDataSourceRequest,
    ) -> Result<String, Error> {
        let query = "SELECT name, host, database, username, password, port, database_path, database_name, database_type FROM data_sources WHERE id = ?";

        let database_params = match query_as::<_, DataSource>(query)
            .bind(payload.id)
            .fetch_one(&self.pool)
            .await {
                Ok(data_source) => data_source,
                Err(e) => {
                    eprintln!("{} {}", "❌ Failed to query data source: ".color("Red"), e);
                    return Err(anyhow::anyhow!("Failed to query data source: {}", e));
                }
            };

        // Здесь можно добавить логику тестирования подключения к источнику данных
        // Например, проверить доступность базы данных, указанной в database_params

        return Ok("Connection test successful".to_string());
    }
}
