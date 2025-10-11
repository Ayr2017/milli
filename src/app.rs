use std::path::PathBuf;
use axum::{Router, routing::get, routing::post, response::IntoResponse, http::{StatusCode, Uri, HeaderMap, header}};
use axum::routing::delete;
use tower_http::services::ServeDir;


// Import controllers
use crate::controllers::api::api_controller::ApiController;
use crate::controllers::api::index_controller::IndexController;

pub async fn create_app() -> Router {
    Router::new()
        // API routes
        .route("/api/test", get(ApiController::test))
        .route("/api/indexes", get(IndexController::index))
        .route("/api/indexes", post(IndexController::store))
        .route("/api/indexes/{:uid}", delete(IndexController::delete))
        .route("/api/indexes/{:uid}", get(IndexController::show))

        // Static resources for SvelteKit (JS, CSS, images)
        .nest_service("/_app", ServeDir::new(PathBuf::from("static/_app")))
        .nest_service("/assets", ServeDir::new(PathBuf::from("static/assets")))

        // All other routes (including /about, /contact, etc.) - SPA fallback
        .fallback(spa_handler)
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
