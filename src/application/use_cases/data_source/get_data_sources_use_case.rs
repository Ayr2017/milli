use axum::extract::State;
use axum::Json;
use colored::Colorize;
use sqlx::query_as;
use crate::domain::repository::data_source_repository_trait::DataSourceRepositoryTrait;
use crate::infrastructure::repositories::data_source_repository::DataSourceRepository;
use crate::state::AppState;

pub struct GetDataSourcesUseCase {
    pub state: State<AppState>,
}

impl GetDataSourcesUseCase {
    pub fn new(state: State<AppState>) -> Self {
        Self {
            state,
        }
    }
    
    pub async fn execute(&self) -> Result<Vec<String>, Json<serde_json::Value>> {
        let pool = match &self.state.database.get_pool_connection().await {
            Ok(pool) => pool,
            Err(e) => {
                eprintln!("{} {}", "Failed to get DB connection: ".color("Red"), e);
                return Err(
                    Json(
                        serde_json::json!(
                        {
                            "code": 500,
                            "success": false,
                            "message": "Database connection error"
                        }    
                        )
                    )
                );
            }
        };
        
        // let data_source_repository = DataSourceRepository::new(pool.clone());

        // let query_result =
        //     query_as::<_, crate::db::data_source::DataSource>("SELECT * FROM data_sources")
        //         .fetch_all(pool)
        //         .await;
        Ok(vec![])
    }

}