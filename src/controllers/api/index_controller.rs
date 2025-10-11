use axum::extract::{Path, Query};
use crate::requests::index::store_index_request::StoreIndexRequest;
use axum::Json;
use axum::response::IntoResponse;
use meilisearch_sdk::client::Client;
use rand::{distr::Alphanumeric, Rng};
use colored::Colorize;
use rusqlite::params;
use crate::requests::index::delete_index_request::DeleteIndexRequest;
use crate::responses::indexes::show_index_response::ShowIndexResponse;

pub struct IndexController;

impl IndexController {
    pub async fn index() -> impl IntoResponse {
        let client = Client::new(
            "http://localhost:7700",
            Some("your_master_key_here_change_this"),
        )
        .unwrap();

        let indexes = client.list_all_indexes().await.unwrap();
        let data = serde_json::json!(
            indexes.results.iter().map(|index| {
                serde_json::json!({
                    "uid": index.uid,
                    "primary_key": index.primary_key, 
                    "created_at": index.created_at,
                })
            }).collect::<Vec<_>>()
        );

        for index in &indexes.results {
            println!(
                "Индекс: {:?}, Первичный ключ: {:?} Всё: {:?}",
                index.uid, index.primary_key, index
            );
        }

        Json(data)
    }

    pub async fn store(Json(payload): Json<StoreIndexRequest>) -> impl IntoResponse {
        let client = Client::new(
            "http://localhost:7700",
            Some("your_master_key_here_change_this"),
        )
        .unwrap();
        
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
        axum::Json(body): axum::Json<DeleteIndexRequest>,
    ) -> impl IntoResponse {
        println!("Params: {:?}", body);
        println!("Удаление индекса: {}", uid);
        
        // Получение конфигурации из переменных окружения
        let meilisearch_url = "http://localhost:7700".to_string();
        let master_key = "your_master_key_here_change_this";
        
        let client = Client::new(&meilisearch_url, Some(master_key)).unwrap();
        
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
    ) -> impl IntoResponse {
        let client = Client::new(
            "http://localhost:7700",
            Some("your_master_key_here_change_this"),
        )
        .unwrap();  
            
        let index_info = client.get_index(uid).await.unwrap();
        let stats = index_info.get_stats().await.unwrap();
        let settings = index_info.get_settings().await.unwrap();
        println!("{:?}", stats);
        let data = serde_json::json!({
        "uid": index_info.uid,
        "created_at": index_info.created_at,
        "updated_at": index_info.updated_at,
        "primary_key": index_info.primary_key,
        "stats": {
            "number_of_documents": stats.number_of_documents,
            "is_indexing": stats.is_indexing,
        },
        "searchable_attributes": settings.searchable_attributes,
        "filterable_attributes": settings.filterable_attributes,
        "sortable_attributes": settings.sortable_attributes,
        "displayable_attributes": settings.displayed_attributes,
        "ranking_rules": settings.ranking_rules,
        "stop_words": settings.stop_words,
        "synonyms": settings.synonyms,
        "distinct_attribute": settings.distinct_attribute,
    });
        // Преобразуем в ShowIndexResponse для форматирования дат
        match ShowIndexResponse::from_json_value(&data) {
            Ok(formatted_response) => {
                Json(serde_json::to_value(formatted_response).unwrap())
            }
            Err(e) => {
                // В случае ошибки возвращаем оригинальный JSON
                eprintln!("Error formatting dates: {}", e);
                Json(data)
            }
        }
    }
}