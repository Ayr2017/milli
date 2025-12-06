use std::ops::Deref;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use crate::requests::index::store_index_request::StoreIndexRequest;
use axum::Json;
use axum::response::IntoResponse;
use colored::Colorize;
use serde_json::json;
use crate::application::use_cases::index::show_index_use_case::ShowIndexUseCase;
use crate::responses::indexes::show_index_response::ShowIndexResponse;
use crate::state::AppState;
use crate::utilits::DateTimeFormatter;

pub struct IndexController {}

impl IndexController {
    pub async fn index(
        State(state): State<AppState>
    ) -> impl IntoResponse {
        
        let client = state.meilisearch_client;

        let indexes = client.list_all_indexes().await.unwrap();
        let data = serde_json::json!(
            indexes.results.iter().map(|index| {
                serde_json::json!({
                    "uid": index.uid,
                    "primary_key": index.primary_key, 
                    "created_at": index.created_at.to_formatted_string(),
                })
            }).collect::<Vec<_>>()
        );
        Json(data)
    }

    pub async fn store(
        State(state): State<AppState>,
        Json(payload): Json<StoreIndexRequest>,        
    ) -> impl IntoResponse {
        let client = state.meilisearch_client;

        println!("Запрос на создание индекса: {:?}", payload);
        
        let task_info = client.create_index(payload.name, payload.pkey.as_deref()).await.unwrap(); 
        Json(serde_json::json!({
            "code": 200,
            "success": true,
            "message": "Index in order to be created",
            "payload": {
                "task_id" : task_info.get_task_uid(),
                "homepage": null
        }}))
    }
    
    pub async fn delete(
        Path(uid): Path<String>,
        State(state): State<AppState>,
    ) -> impl IntoResponse {
        let client = state.meilisearch_client;
        
        match client.delete_index(uid.clone()).await {
            Ok(task_info) => {
                println!("{:?} {}", task_info, "Индекс успешно удалён".color(colored::Color::Green));
                Json(serde_json::json!({
                    "code": 200,
                    "success": true,
                    "message": "Индекс помечен к удалению",
                    "payload": {
                        "task_id": task_info.get_task_uid(),
                        "uid": uid
                    }
                }))
            }
            Err(e) => {
                eprintln!("Ошибка удаления индекса {}: {:?}", uid, e);
                println!("{} {}", "{}".color(colored::Color::Red), e);
                Json(serde_json::json!({
                    "code": 400,
                    "success": false,
                    "message": "Не удалось удалить индекс",
                    "error": format!("Индекс '{}' не найден или произошла ошибка", uid)
                }))
            }
        }
    }
    
    pub async fn show(
        Path(uid): Path<String>,
        State(state): State<AppState>,
    ) -> impl IntoResponse {
        println!("{}",uid.clone().to_string());
        let client_ref = state.meilisearch_client.as_ref();
        let show_index_use_case = ShowIndexUseCase::new(client_ref.clone(), uid.clone());

        match show_index_use_case.execute().await {
            Ok(data) => Json(data).into_response(),
            Err(e) => {
                eprintln!("Ошибка получения индекса: {:?}", e);
                let error_response = serde_json::json!({
                    "code": 500,
                    "success": false,
                    "message": "Не удалось получить индекс",
                    "error": format!("Индекс '{}' не найден или произошла ошибка", uid.clone())
            });
                (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)).into_response()
            }
        }
    }
}