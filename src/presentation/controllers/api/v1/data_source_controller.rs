use axum::extract::{Path, State};
use axum::Json;
use axum::response::IntoResponse;
use colored::Colorize;
use sqlx::query_as;
use crate::application::use_cases::data_source::test_data_sources_use_case::TestDataSourcesUseCase;
use crate::presentation::requests::data_sources::index_data_source_request::IndexDataSourceRequest;
use crate::state::AppState;

pub struct DataSourceController {
    use_case: TestDataSourcesUseCase,
}


impl DataSourceController {
    fn new (use_case: TestDataSourcesUseCase) -> Self {
        Self{
            use_case
        }
    }

    pub async fn index(
        State(state): State<AppState>,
        Json(payload): Json<IndexDataSourceRequest>,
    ) -> impl IntoResponse {
        let pool = match state.database.get_pool_connection().await {
            Ok(pool) => pool,
            Err(e) => {
                eprintln!("{} {}", "Failed to get DB connection: ".color("Red"), e);
                return Json(serde_json::json!({
                    "code": 500,
                    "success": false,
                    "message": "Database connection error"
                }));
            }
        };
        // Выполняем запрос с использованием sqlx
        let query_result =
            query_as::<_, crate::db::data_source::DataSource>("SELECT * FROM data_sources")
                .fetch_all(pool)
                .await;

        match query_result {
            Ok(data_sources) => {
                let ds: Vec<serde_json::Value> = data_sources
                    .iter()
                    .map(|ds| {
                        serde_json::json!({
                            "id": ds.id,
                            "name": ds.name,
                            "host": ds.host,
                            "database": ds.database,
                            "username": ds.username,
                            // НЕ возвращаем пароль из соображений безопасности
                            "port": ds.port,
                            "database_path": ds.database_path,
                            "database_name": ds.database_name,
                            "database_type": ds.database_type,
                            "created_at": ds.created_at,
                            "updated_at": ds.updated_at
                        })
                    })
                    .collect();

                println!("Indexing data source");
                Json(serde_json::json!({
                    "code": 200,
                    "success": true,
                    "message": "Data source is here",
                    "data_sources": ds,
                    "payload": {
                        "task_id" : "1234567890",
                        "homepage": null,
                    }
                }))
            }
            Err(e) => {
                eprintln!("Failed to execute query: {:?}", e);
                Json(serde_json::json!({
                    "code": 500,
                    "success": false,
                    "message": "Database query execution error"
                }))
            }
        }
    }
}
