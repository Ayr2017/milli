use crate::requests::data_source::store_data_source_request::StoreDataSourceRequest;
use crate::state::AppState;
use anyhow::Context;
use axum::Json;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use chrono::Utc;
use colored::Color::Red;
use colored::Colorize;
use rusqlite::fallible_iterator::FallibleIterator;

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

    pub async fn show(State(state): State<AppState>, Path(id): Path<u32>) -> impl IntoResponse {}

    pub async fn store(
        State(state): State<AppState>,
        Json(payload): Json<StoreDataSourceRequest>,
    ) -> impl IntoResponse {
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
        let mut stmt = match conn.prepare("INSERT INTO data_sources (name, host, database, username, port, database_path, database_name, database_type, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)") {
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

        let current_time = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

        let result = match stmt.execute(&[
            payload.name.as_str(),
            payload.host.as_str(),
            payload.database.as_str(),
            payload.username.as_str(),
            payload.port.to_string().as_str(),
            payload.database_path.as_str(),
            payload.database_name.as_str(),
            payload.database_type.as_str(),
            current_time.as_str(),
            current_time.as_str(),
        ]) {
            Ok(_) => {
                // Возвращаем успешный JSON-ответ
                Json(serde_json::json!({
                    "code": 201,
                    "success": true,
                    "message": "Data source created successfully"
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
        };
        result
    }

    pub async fn update(State(state): State<AppState>, Path(id): Path<u32>) -> impl IntoResponse {}

    pub async fn destroy(State(state): State<AppState>, Path(id): Path<u32>) {}
}
