use anyhow::Context;
use crate::state::AppState;
use axum::Json;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use colored::Color::Red;
use colored::Colorize;
use rusqlite::fallible_iterator::FallibleIterator;

pub struct DataSourceController {}

impl DataSourceController {
    pub async fn index(State(state): State<AppState>) -> impl IntoResponse {
        let conn = match state.database.get_pool_connection()
            .await {
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
}
