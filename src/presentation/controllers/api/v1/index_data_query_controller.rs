use axum::extract::{ Query, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde_json::json;
use crate::application::use_cases::index_data_query::test_index_data_query_use_case::TestIndexDataQueryUseCase;
use crate::domain::repository::data_source_repository_trait::DataSourceRepositoryTrait;
use crate::requests::index_data_query::index_index_data_query_request::IndexIndexDataQueryRequest;
use crate::requests::index_data_query::test_index_data_query_request::TestIndexDataQueryRequest;
use crate::infrastructure::repositories::data_source_repository::DataSourceRepository;
use crate::state::AppState;

pub struct IndexDataQueryController {}

impl IndexDataQueryController {
    
    pub async fn new() -> Self {
        Self {}
    }
    pub async fn test(
        payload: Query<TestIndexDataQueryRequest>,
        State(state): State<AppState>
    ) -> impl IntoResponse{
        let db = (*state.database).clone();
        let repository = DataSourceRepository::new(db);
        let test_index_data_query_use_case = TestIndexDataQueryUseCase::new(repository);

        let result = test_index_data_query_use_case.execute(&payload).await;

        match result {
            Ok(json_value) => (
                StatusCode::OK,
                Json(json!({
                "code": 200,
                "success": true,
                "message": "Query executed successfully",
                "result": json_value, // Вместо format!("{:?}", result)
                "data": format!("{:?}", payload.0)
            })),
            ),
            Err(error_msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                "code": 500,
                "success": false,
                "message": error_msg,
                "result": null,
                "data": format!("{:?}", payload)
            })),
            )
        }

    }

    pub async fn index(
        Query(params): Query<IndexIndexDataQueryRequest>,
    ) -> impl IntoResponse{
        println!("{:?}", params);
        (
            StatusCode::OK,
            Json(json!({
                    "code": 200,
                    "success": true,
                    "message": "Database connection error",
                    "data": format!("{:?}", params)
                    })),
        )
    }
}