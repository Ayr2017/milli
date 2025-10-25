use crate::requests::data_source::store_data_source_request::StoreDataSourceRequest;
use crate::state::AppState;
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use colored::Colorize;
use sqlx::{query, query_as};
use serde_json::json;
use crate::requests::data_source::test_data_source_request::TestDataSourceRequest;
use crate::services::data_source_service::DataSourceService;

pub struct DataSourceController {}

impl DataSourceController {
    pub async fn index(State(state): State<AppState>) -> impl IntoResponse {
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
        let query_result = query_as::<_, crate::db::data_source::DataSource>(
            "SELECT * FROM data_sources"
        )
        .fetch_all(pool)
        .await;

        match query_result {
            Ok(data_sources) => {
                let ds: Vec<serde_json::Value> = data_sources.iter().map(|ds| {
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
                }).collect();

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
            },
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

    pub async fn store(
        State(state): State<AppState>,
        Json(payload): Json<StoreDataSourceRequest>,
    ) -> impl IntoResponse {
        println!("🔍 Received payload: {:#?}", payload);

        // Проверим валидность данных
        if payload.name.is_empty() {
            println!("❌ Validation error: name is empty");
            return (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(json!({
                    "code": 422,
                    "success": false,
                    "message": "Validation failed",
                    "errors": {
                        "name": ["Name field is required and cannot be empty"]
                    }
                })),
            );
        }

        if payload.host.is_empty() {
            println!("❌ Validation error: host is empty");
            return (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(json!({
                    "code": 422,
                    "success": false,
                    "message": "Validation failed",
                    "errors": {
                        "host": ["Host field is required and cannot be empty"]
                    }
                })),
            );
        }

        if payload.port == 0 || payload.port > 65535 {
            println!("❌ Validation error: invalid port {}", payload.port);
            return (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(json!({
                    "code": 422,
                    "success": false,
                    "message": "Validation failed",
                    "errors": {
                        "port": ["Port must be between 1 and 65535"]
                    }
                })),
            );
        }

        if !["sqlite", "mysql", "postgresql"].contains(&payload.database_type.as_str()) {
            println!(
                "❌ Validation error: invalid database_type {}",
                payload.database_type
            );
            return (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(json!({
                    "code": 422,
                    "success": false,
                    "message": "Validation failed",
                    "errors": {
                        "database_type": ["Database type must be sqlite, mysql, or postgresql"]
                    }
                })),
            );
        }

        let pool = match state.database.get_pool_connection().await {
            Ok(pool) => pool,
            Err(e) => {
                eprintln!("{} {}", "❌ Failed to get DB connection: ".color("Red"), e);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "code": 500,
                        "success": false,
                        "message": "Database connection error",
                        "error": format!("{}", e)
                    })),
                );
            }
        };

        let sql = "INSERT INTO data_sources (name, host, database, username, password, port, database_path, database_name, database_type) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)";

        // Выполняем запрос с использованием sqlx
        let result = query(sql)
            .bind(&payload.name)
            .bind(&payload.host)
            .bind(&payload.database)
            .bind(&payload.username)
            .bind(&payload.password)
            .bind(payload.port)
            .bind(&payload.database_path)
            .bind(&payload.database_name)
            .bind(&payload.database_type)
            .execute(pool)
            .await;

        match result {
            Ok(result) => (
                StatusCode::CREATED,
                Json(json!({
                    "code": 201,
                    "success": true,
                    "message": "Data source created successfully",
                    "rows_affected": result.rows_affected()
                })),
            ),
            Err(e) => {
                eprintln!("❌ Failed to execute query: {:?}", e);
                eprintln!("❌ Error message: {}", e);

                // Проверим конкретные типы ошибок SQLite
                let error_message = match &e {
                    sqlx::Error::Database(db_err) if db_err.message().contains("UNIQUE constraint failed") => {
                        "Constraint violation: check your data types and constraints".to_string()
                    }
                    sqlx::Error::Database(db_err) if db_err.message().contains("schema has changed") => {
                        "Database schema has changed".to_string()
                    }
                    _ => format!("Database execution error: {}", e),
                };

                (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    Json(json!({
                        "code": 422,
                        "success": false,
                        "message": "Database execution failed",
                        "error": error_message
                    })),
                )
            }
        }
    }

    pub async fn destroy(
        Path(id): Path<String>,
        State(state): State<AppState>,
    ) -> impl IntoResponse {
        let pool = match state.database.get_pool_connection().await {
            Ok(pool) => pool,
            Err(e) => {
                eprintln!("{} {}", "❌ Failed to get DB connection: ".color("Red"), e);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "code": 500,
                        "success": false,
                        "message": "Database connection error",
                        "error": format!("{}", e)
                    })),
                );
            }
        };

        // Выполняем запрос с использованием sqlx
        let result = query("DELETE FROM data_sources WHERE id = ?")
            .bind(&id)
            .execute(pool)
            .await;

        match result {
            Ok(result) => {
                // Проверяем, была ли удалена хотя бы одна строка
                let rows_affected = result.rows_affected();
                if rows_affected > 0 {
                    // Успешное удаление
                    (
                        StatusCode::OK,
                        Json(json!({
                            "code": 200,
                            "success": true,
                            "message": "Data source deleted successfully",
                            "rows_affected": rows_affected
                        })),
                    )
                } else {
                    // Запись с таким ID не найдена
                    (
                        StatusCode::NOT_FOUND,
                        Json(json!({
                            "code": 404,
                            "success": false,
                            "message": "Data source not found"
                        })),
                    )
                }
            }
            Err(e) => {
                // Обрабатываем ошибку выполнения запроса
                eprintln!("{} {}", "❌ Failed to delete data source: ".color("Red"), e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "code": 500,
                        "success": false,
                        "message": "Failed to delete data source",
                        "error": format!("{}", e)
                    })),
                )
            }
        }
    }

    pub async fn show(Path(id): Path<u32>, State(state): State<AppState>) -> impl IntoResponse {
        let pool = match state.database.get_pool_connection().await {
            Ok(pool) => pool,
            Err(e) => {
                eprintln!("{} {}", "❌ Failed to get DB connection: ".color("Red"), e);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                    "code": 500,
                    "success": false,
                    "message": "Database connection error",
                    "error": format!("{}", e)
                    })),
                );
            }
        };

        // Выполняем запрос с использованием sqlx
        let result = query_as::<_, crate::db::data_source::DataSource>(
            "SELECT * FROM data_sources WHERE id = ? LIMIT 1"
        )
        .bind(id)
        .fetch_optional(pool)
        .await;

        match result {
            Ok(Some(data_source)) => {
                // Успешное получение данных
                let data_source_json = json!({
                    "id": data_source.id,
                    "name": data_source.name,
                    "host": data_source.host,
                    "database": data_source.database,
                    "username": data_source.username,
                    "password": "********", // Не возвращаем пароль из соображений безопасности
                    "port": data_source.port,
                    "database_path": data_source.database_path,
                    "database_name": data_source.database_name,
                    "database_type": data_source.database_type,
                    "created_at": data_source.created_at,
                });

                (
                    StatusCode::OK,
                    Json(json!({
                        "code": 200,
                        "success": true,
                        "message": "Data source found",
                        "data_source": data_source_json
                    })),
                )
            }
            Ok(None) => {
                // Запись не найдена
                (
                    StatusCode::NOT_FOUND,
                    Json(json!({
                        "code": 404,
                        "success": false,
                        "message": "Data source not found"
                    })),
                )
            }
            Err(e) => {
                eprintln!("{} {}", "❌ Failed to query row: ".color("Red"), e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!(
                        {
                            "code": 500,
                            "success": false,
                            "message": "Failed to query row",
                            "error": format!("{}", e)
                        }
                    )),
                )
            }
        }
    }

    pub async fn update() {}

    pub async fn test(
        State(state): State<AppState>,
        Json(payload): Json<TestDataSourceRequest>,
    ) -> impl IntoResponse {
        let pool = match state.database.get_pool_connection().await {
            Ok(pool) => pool,
            Err(e) => {
                eprintln!("{} {}", "❌ Failed to get DB connection: ".color("Red"), e);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                    "code": 500,
                    "success": false,
                    "message": "Database connection error",
                    "error": format!("{}", e)
                })),
                );
            }
        };

        // Создаем сервис с клонированным пулом соединений
        let data_source_service = DataSourceService::new(pool.clone());

        // Вызываем метод test_data_source и ждем результат
        match data_source_service.test_data_source(payload).await {
            Ok(result) => (
                StatusCode::OK,
                Json(json!({
                    "code": 200,
                    "success": true,
                    "message": "Data source tested successfully",
                    "result": result
                })),
            ),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "code": 500,
                    "success": false,
                    "message": "Failed to test data source",
                    "error": format!("{}", e)
                })),
            ),
        }
    }
}
