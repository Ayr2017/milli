mod config;
mod controllers;
mod app;

use axum::{Router, routing::get, response::{Html, IntoResponse}, Json, http::{StatusCode, Uri, HeaderMap, header}};
use axum::routing::post;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let app = Router::new()
        // API маршруты
        .route("/api/test", get(api_test))
        .route("/api/indexes", get(index_index))
        .route("/api/indexes_", post(create_index))

        // Статические ресурсы SvelteKit (JS, CSS, изображения)
        .nest_service("/_app", ServeDir::new("static/_app"))
        .nest_service("/assets", ServeDir::new("static/assets"))
        
        // ВСЕ остальные маршруты (включая /about, /contact и т.д.) - SPA fallback
        .fallback(spa_handler);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Server running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn spa_handler(uri: Uri) -> impl IntoResponse {
    let path = uri.path();
    
    // Если это API запрос, который не был обработан выше
    if path.starts_with("/api") {
        return (StatusCode::NOT_FOUND, "API endpoint not found").into_response();
    }
    
    println!("Serving SPA for path: {}", path);
    
    // ДЛЯ ВСЕХ остальных маршрутов (/, /about, /contact и т.д.) 
    // возвращаем один и тот же index.html
    // SvelteKit на клиенте сам определит какой компонент показать

    let file_path = if path == "/" {
        "static/index.html".to_string()
    } else {
        // Убираем начальный слэш и добавляем /index.html
        format!("static{}.html", path)
    };

    match std::fs::read_to_string(&file_path) {
        Ok(content) => {
            let mut headers = HeaderMap::new();
            headers.insert(header::CONTENT_TYPE, "text/html; charset=utf-8".parse().unwrap());
            (StatusCode::OK, headers, content).into_response()
        },
        Err(err) => {
            eprintln!("Failed to read {}: {}", file_path, err);

            if err.kind() == std::io::ErrorKind::NotFound {
                // Пытаемся прочитать кастомную страницу 404
                match std::fs::read_to_string("static/404.html") {
                    Ok(html_content) => {
                        let mut headers = HeaderMap::new();
                        headers.insert(header::CONTENT_TYPE, "text/html; charset=utf-8".parse().unwrap());
                        (StatusCode::NOT_FOUND, headers, html_content).into_response()
                    },
                    Err(_) => {
                        // Если файл 404.html не найден, возвращаем стандартную страницу
                        (StatusCode::NOT_FOUND, "Page not found p-").into_response()
                    }
                }
            } else {
                (StatusCode::INTERNAL_SERVER_ERROR, "Failed to load application").into_response()
            }
        }
    }
}

async fn api_test() -> impl IntoResponse {
    Json(serde_json::json!({
        "message": "Hello, Updated World!!!"
    }))
}
async fn create_index() -> impl IntoResponse {
    Json(serde_json::json!({
        "message": "Index created successfully!"
    }))
}

async fn index_index() -> impl IntoResponse {
    Json(serde_json::json!({
        "data": [
            {"uid":"customers","name": "Index 1", "searchable": [],"filterable": [], "sortable": []},
            {"uid":"customers","name": "Index 2", "searchable": [],"filterable": [], "sortable": []},
        ]
    }))
}