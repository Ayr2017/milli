use crate::requests::data_source::store_data_source_request::StoreDataSourceRequest;
use crate::state::AppState;
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use colored::Colorize;
use rusqlite::fallible_iterator::FallibleIterator;
use serde_json::json;

pub struct DataSourceController {}

impl DataSourceController {
    pub async fn index(State(state): State<AppState>) -> impl IntoResponse {
        let conn = match state.database.get_pool_connection().await {
            Ok(connection) => connection,
            Err(e) => {
                eprintln!("{} {}", "Failed to get DB connection: ".color("Red"), e);
                return Json(serde_json::json!({
                    "code": 500,
                    "success": false,
                    "message": "Database connection error"
                }));
            }
        };

        // Безопасная подготовка запроса
        let mut stmt = match conn.prepare("SELECT id, name, host, database, username, port, database_path, database_name, database_type, created_at, updated_at FROM data_sources") {
            Ok(stmt) => stmt,
            Err(e) => {
                eprintln!("Failed to prepare statement: {:?}", e);
                return Json(serde_json::json!({
                "code": 500,
                "success": false,
                "message": "Database query error"
            }));
            }
        };
        let ds_iter = match stmt.query_map([], |row| {
            Ok(serde_json::json!({
                "id": row.get::<_, u32>(0)?,
                "name": row.get::<_, String>(1)?,
                "host": row.get::<_, String>(2)?,
                "database": row.get::<_, String>(3)?,
                "username": row.get::<_, String>(4)?,
                // НЕ возвращаем пароль из соображений безопасности
                "port": row.get::<_, u16>(5)?,
                "database_path": row.get::<_, String>(6)?,
                "database_name": row.get::<_, String>(7)?,
                "database_type": row.get::<_, String>(8)?,
                "created_at": row.get::<_, String>(9)?,
                "updated_at": row.get::<_, String>(10)?
            }))
        }) {
            Ok(iter) => iter,
            Err(e) => {
                eprintln!("Failed to execute query: {:?}", e);
                return Json(serde_json::json!({
                    "code": 500,
                    "success": false,
                    "message": "Database query execution error"
                }));
            }
        };

        let mut ds = Vec::new();
        for user in ds_iter {
            ds.push(user.unwrap());
        }

        println!("Indexing data source");
        Json(serde_json::json!({
            "code": 200,
            "success": true,
            "message": "Data source is here",
            "data_sources": ds,
            "payload": {
                "task_id" : "1234567890",
                "homepage": null,
        }}))
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

        let conn = match state.database.get_pool_connection().await {
            Ok(connection) => connection,
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

        let sql = "INSERT INTO data_sources (name, host, database, username, password, port, database_path, database_name, database_type) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)";

        let mut stmt = match conn.prepare(sql) {
            Ok(stmt) => stmt,
            Err(e) => {
                eprintln!("❌ Failed to prepare statement: {:?}", e);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "code": 500,
                        "success": false,
                        "message": "Database query preparation error",
                        "error": format!("{}", e)
                    })),
                );
            }
        };

        // Подробное логирование параметров
        let params = [
            &payload.name,
            &payload.host,
            &payload.database,
            &payload.username,
            &payload.password,
            &payload.port.to_string(),
            &payload.database_path,
            &payload.database_name,
            &payload.database_type,
        ];

        let result = match stmt.execute(&params) {
            Ok(rows_affected) => (
                StatusCode::CREATED,
                Json(json!({
                    "code": 201,
                    "success": true,
                    "message": "Data source created successfully",
                    "rows_affected": rows_affected
                })),
            ),
            Err(e) => {
                eprintln!("❌ Failed to execute query: {:?}", e);
                eprintln!("❌ Error kind: {:?}", e.sqlite_error_code());
                eprintln!("❌ Error message: {}", e);

                // Проверим конкретные типы ошибок SQLite
                let error_message = match e.sqlite_error_code() {
                    Some(rusqlite::ErrorCode::ConstraintViolation) => {
                        "Constraint violation: check your data types and constraints".to_string()
                    }
                    Some(rusqlite::ErrorCode::SchemaChanged) => {
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
                        "error": error_message,
                        "sqlite_error_code": format!("{:?}", e.sqlite_error_code())
                    })),
                )
            }
        };
        result
    }

    pub async fn destroy(
        Path(id): Path<String>,
        State(state): State<AppState>,
    ) -> impl IntoResponse {
        let conn = match state.database.get_pool_connection().await {
            Ok(connection) => connection,
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

        match conn.execute("DELETE FROM data_sources WHERE id = ?", [id]) {
            Ok(rows_affected) => {
                // Проверяем, была ли удалена хотя бы одна строка
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

    pub async fn show(Path(id): Path<String>, State(state): State<AppState>) -> impl IntoResponse {
        let conn = match state.database.get_pool_connection().await {
            Ok(connection) => connection,
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

        match conn.query_row(
            "SELECT * FROM data_sources WHERE id = ? limit 1",
            [&id],
            |row| {
                Ok(json!({
                "id": row.get::<_, String>("id")?,
                "name": row.get::<_, String>("name")?,
                "url": row.get::<_, String>("url")?,
                "created_at": row.get::<_, String>("created_at")?,
                // Добавьте другие поля вашей таблицы
            }))
            },
        ) {
            Ok(data_source) => {
                // Успешное получение данных
                (StatusCode::OK,
                Json(json!({
                    "code": 200,
                    "success": true,
                    "message": "Data source found",
                    "data_source": data_source
                })))
            },
            Err(e) => {
                eprintln!("{} {}", "❌ Failed to query row: ".color("Red"), e);
                (StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!(
                    {
                        "code": 500,
                        "success": false,
                        "message": "Failed to query row",
                        "error": format!("{}", e)
                    }
                )))
            }
        }
    }

pub async fn update() {}
}
