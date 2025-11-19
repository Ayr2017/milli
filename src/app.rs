use std::path::PathBuf;
use axum::{
    Router,
    routing::get,
    routing::post,
    response::IntoResponse,
    http::{
        StatusCode,
        Uri,
        HeaderMap,
        header
    },
    extract::ws::{
        WebSocketUpgrade,
        WebSocket
    },
};
use axum::routing::delete;
use tower_http::services::ServeDir;


// Import controllers
use crate::controllers::api::api_controller::ApiController;
use crate::controllers::api::index_controller::IndexController;
use crate::controllers::api::data_source_controller::DataSourceController;
use crate::presentation::controllers::api::v1::index_data_query_controller::IndexDataQueryController;
use crate::presentation::controllers::api::v1::ws_controller::WsController;
use crate::state::AppState;

pub async fn create_app(
    state: AppState
) -> Router {
    
    Router::new()
        // API routes
        .route("/api/test", get(ApiController::test))
        .route("/api/indexes", get(IndexController::index))
        .route("/api/indexes", post(IndexController::store))
        .route("/api/indexes/{:uid}", delete(IndexController::delete))
        .route("/api/indexes/{:uid}", get(IndexController::show))
        .route("/api/data-sources", get(DataSourceController::index))
        .route("/api/data-sources", post(DataSourceController::store))
        .route("/api/data-sources/test", post(DataSourceController::test))
        .route("/api/data-sources/{:id}", delete(DataSourceController::destroy))
        .route("/api/data-sources/{:id}", get(DataSourceController::show))
        .route("/api/index-data-queries", get(IndexDataQueryController::index))
        .route("/api/index-data-queries/test", get(IndexDataQueryController::test))
        .route("/api/index-data-queries", post(IndexDataQueryController::store))
        .route("/api/index-data-queries/insert-data", post(IndexDataQueryController::insert_data))
        .route("/ws", get(WsController::websocket_handler))
        // Static resources for SvelteKit (JS, CSS, images)
        .nest_service("/_app", ServeDir::new(PathBuf::from("static/_app")))
        .nest_service("/assets", ServeDir::new(PathBuf::from("static/assets")))

        // All other routes (including /about, /contact, etc.) - SPA fallback
        .fallback(spa_handler)
        .with_state(state)
}

/**

    .layer(middleware::from_fn(request_logging_middleware))
    .layer(TraceLayer::new_for_http())
*/
async fn request_logging_middleware(
    request: axum::extract::Request,
    next: axum::middleware::Next,
) -> axum::response::Response {



    let method = request.method().clone();
    let uri = request.uri().clone();
    
    // Логируем входящий запрос
    println!("🚀 {} {} - Incoming request", method, uri);
    
    // Если это POST запрос к API, попробуем залогировать тело запроса
    if method == axum::http::Method::POST && uri.path().starts_with("/api") {
        println!("📝 POST request to API endpoint: {}", uri.path());
    }
    
    let response = next.run(request).await;
    
    // Логируем статус ответа
    println!("📤 {} {} - Response status: {}", method, uri, response.status());
    
    response
}

async fn spa_handler(uri: Uri) -> impl IntoResponse {
    let path = uri.path();

    // If this is an API request that wasn't handled above
    if path.starts_with("/api") {
        return (StatusCode::NOT_FOUND, "API endpoint not found").into_response();
    }

    println!("Serving SPA for path: {}", path);

    // For ALL other routes (/, /about, /contact, etc.)
    // return the same index.html
    // SvelteKit on the client will determine which component to show

    let file_path = if path == "/" {
        "static/index.html".to_string()
    } else {
        // Remove the initial slash and add /index.html
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
                // Try to read the custom 404 page
                match std::fs::read_to_string("static/404.html") {
                    Ok(html_content) => {
                        let mut headers = HeaderMap::new();
                        headers.insert(header::CONTENT_TYPE, "text/html; charset=utf-8".parse().unwrap());
                        (StatusCode::NOT_FOUND, headers, html_content).into_response()
                    },
                    Err(_) => {
                        // If the 404.html file is not found, return a standard page
                        (StatusCode::NOT_FOUND, "Page not found p-").into_response()
                    }
                }
            } else {
                (StatusCode::INTERNAL_SERVER_ERROR, "Failed to load application").into_response()
            }
        }
    }
}