use axum::extract::{Path, State};
use crate::requests::index::store_index_request::StoreIndexRequest;
use axum::Json;
use axum::response::IntoResponse;
use colored::Colorize;
use crate::responses::indexes::show_index_response::ShowIndexResponse;
use crate::state::AppState;

pub struct DataSourceController {}

impl DataSourceController {
    pub async fn index(
        State(state): State<AppState>
    )-> impl IntoResponse {
        println!("Indexing data source");
        Json(serde_json::json!({
            "code": 200,
            "success": true,
            "message": "Data source is here",
            "payload": {
                "task_id" : "1234567890",
                "homepage": null
        }}))
    
    }
}