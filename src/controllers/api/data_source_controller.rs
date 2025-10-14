use crate::database::Database;
use crate::requests::index::store_index_request::StoreIndexRequest;
use crate::responses::indexes::show_index_response::ShowIndexResponse;
use crate::state::AppState;
use axum::Json;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use colored::Colorize;

pub struct DataSourceController {}

impl DataSourceController {
    pub async fn index(State(state): State<AppState>) -> impl IntoResponse {
        let conn = state.database.unwrap().get_pool_connection().unwrap();
        let mut stmt = conn.prepare("SELECT * FROM data_sources").unwrap();
        let ds_iter = stmt
            .query_map([], |row| {
                Ok(serde_json::json!({
                    "id": row.get::<_, u32>(0)?,
                    "name": row.get::<_, String>(1)?,
                    "host": row.get::<_, String>(2)?,
                    "database": row.get::<_, String>(3)?,
                    "username": row.get::<_, String>(4)?,
                    "password": row.get::<_, String>(5)?,
                    "port": row.get::<_, u16>(6)?,
                    "database_path": row.get::<_, String>(7)?,
                    "database_name": row.get::<_, String>(8)?,
                    "database_type": row.get::<_, String>(9)?,
                    "created_at": row.get::<_, String>(10)?,
                    "updated_at": row.get::<_, String>(11)?
                }))
            })
            .unwrap();

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
