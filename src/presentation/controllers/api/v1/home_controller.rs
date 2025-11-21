use crate::application::use_cases::home::get_system_info_use_case::GetSystemInfoUseCase;
use crate::state::AppState;
use axum::Json;
use axum::extract::State;
use axum::response::IntoResponse;
use serde_json::json;

pub struct HomeController {
    state: AppState,
}

impl HomeController {
    pub async fn index(State(state): State<AppState>) -> impl IntoResponse {
        let system_use_case = GetSystemInfoUseCase::new().await;
        let system_info = system_use_case.execute().await;
        let system_info = system_info.unwrap();
        Json(json!({
            "success": true,
            "data": system_info
        }))
    }
}
