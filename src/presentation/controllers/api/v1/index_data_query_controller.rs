use axum::extract::{ Query, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde_json::json;
use crate::application::use_cases::index_data_query::test_index_data_query_use_case::TestIndexDataQueryUseCase;
use crate::requests::index_data_query::index_index_data_query_request::IndexIndexDataQueryRequest;
use crate::requests::index_data_query::test_index_data_query_request::TestIndexDataQueryRequest;

pub struct IndexDataQueryController {}

impl IndexDataQueryController {
    
    pub async fn new() -> Self {
        Self {}
    }
    pub async fn test(
        Query(params): Query<TestIndexDataQueryRequest>,
        test_index_data_query_use_case: TestIndexDataQueryUseCase,
    ) -> impl IntoResponse{
        let result = test_index_data_query_use_case.execute(params).await;

        println!("{:?}", result);
        (
            StatusCode::OK,
            Json(json!({
                    "code": 200,
                    "success": true,
                    "message": "
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