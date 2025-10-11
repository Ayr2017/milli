use axum::response::IntoResponse;
use axum::Json;

pub struct ApiController;

impl ApiController {
    pub async fn test() -> impl IntoResponse {
        Json(serde_json::json!({
            "message": "Hello, Updated World!!!"
        }))
    }
}
