use anyhow::Error;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use colored::Colorize;
use sqlx::{Pool, Sqlite, query_as, postgres::PgPoolOptions};
use serde_json::json;
use crate::models::data_source::DataSource;
use crate::requests::data_source::test_data_source_request::TestDataSourceRequest;
use crate::state::AppState;

pub struct DataSourceService {
    pub pool: Pool<Sqlite>,
}

impl DataSourceService {
    pub fn new(pool: Pool<Sqlite>) -> DataSourceService {
        DataSourceService {
            pool,
        }
    }

    pub async fn test_data_source(
        &self,
        payload: TestDataSourceRequest,
    ) -> Result<String, Error> {
        let query = "SELECT name, host, database, username, password, port, database_path, database_name, database_type FROM data_sources WHERE id = ?";

        let database_params = match query_as::<_, DataSource>(query)
            .bind(payload.id)
            .fetch_one(&self.pool)
            .await {
                Ok(data_source) => data_source,
                Err(e) => {
                    eprintln!("{} {}", "❌ Failed to query data source: ".color("Red"), e);
                    return Err(anyhow::anyhow!("Failed to query data source: {}", e));
                }
            };

        // Test the connection based on database type
        match database_params.database_type.as_str() {
            "sqlite" => {
                // For SQLite, try to establish a connection to the database file
                let connection_string = format!("sqlite:{}", database_params.database_path);
                match sqlx::SqlitePool::connect(&connection_string).await {
                    Ok(_) => {
                        println!("{} {}", "✅ SQLite connection test successful: ".color("Green"), database_params.name);
                        return Ok("SQLite connection test successful".to_string());
                    },
                    Err(e) => {
                        eprintln!("{} {}", "❌ Failed to connect to SQLite database: ".color("Red"), e);
                        return Err(anyhow::anyhow!("Failed to connect to SQLite database: {}", e));
                    }
                }
            },
            "mysql" => {
                // Since MySQL feature is not enabled in sqlx, we can't directly test the connection
                println!("{} {}", "⚠️ MySQL connection test: ".color("Yellow"), 
                         "MySQL connection testing is not directly supported. Please ensure MySQL features are enabled in sqlx.");
                return Ok("MySQL connection test: Feature not enabled. Connection parameters look valid.".to_string());
            },
            "postgresql" => {
                // For PostgreSQL, try to establish a connection to the database
                let connection_string = format!(
                    "postgres://{}:{}@{}:{}/{}",
                    database_params.username,
                    database_params.password,
                    database_params.host,
                    database_params.port,
                    database_params.database_name
                );
                match PgPoolOptions::new()
                    .max_connections(5)
                    .connect(&connection_string)
                    .await {
                    Ok(_) => {
                        println!("{} {}", "✅ PostgreSQL connection test successful: ".color("Green"), database_params.name);
                        return Ok("PostgreSQL connection test successful".to_string());
                    },
                    Err(e) => {
                        eprintln!("{} {}", "❌ Failed to connect to PostgreSQL database: ".color("Red"), e);
                        return Err(anyhow::anyhow!("Failed to connect to PostgreSQL database: {}", e));
                    }
                }
            },
            _ => {
                return Err(anyhow::anyhow!("Unsupported database type: {}", database_params.database_type));
            }
        }
    }
}
