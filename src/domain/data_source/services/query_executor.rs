use colored::Colorize;
use crate::domain::data_source::entities::data_source::DataSource;
use serde_json::Value;
use sqlx::postgres::{PgPool, PgRow};
use sqlx::{Column, Row, TypeInfo};
use crate::domain::data_source::entities::index_data_query::IndexDataQuery;

pub struct QueryExecutor;

impl QueryExecutor {
    pub fn new() -> Self {
        Self
    }

    /// Выполняет запрос с лимитом для тестирования (возвращает одну запись)
    pub async fn execute_test_query(
        &self,
        data_source: &DataSource,
        query: &str,
    ) -> Result<Value, String> {
        let results = &self.execute_query(&data_source, &query, 1).await?;
        match results.into_iter().next() {
            Some(row) => Ok(row.clone()),
            None => Err("No results found".to_string()),
        }
    }

    /// Выполняет запрос и возвращает все записи или с указанным лимитом
    pub async fn execute_query(
        &self,
        data_source: &DataSource,
        query: &str,
        limit: u32,
    ) -> Result<Vec<Value>, String> {
        let final_query = Self::prepare_query(query, limit);
        let connection_string = Self::build_connection_string(data_source);

        println!("Connection string: {}", connection_string.color("blue"));
        println!("Executing query: {}", final_query.color("yellow"));

        let pool = PgPool::connect(&connection_string)
            .await
            .map_err(|e| format!("Database connection error: {}", e))?;

        let rows = sqlx::query(&final_query)
            .fetch_all(&pool)
            .await
            .map_err(|e| format!("Query execution error: {}", e))?;

        // Закрываем подключение
        pool.close().await;

        Ok(rows.into_iter().map(Self::row_to_json).collect())
    }

    /// Выполняет запрос и возвращает только первую запись
    pub async fn execute_single_query(
        data_source: &DataSource,
        query: &str,
    ) -> Result<Value, String> {
        let query_without_semicolon = query.strip_suffix(";").unwrap_or(query);
        let limited_query = format!("{} LIMIT 1", query_without_semicolon);
        let connection_string = Self::build_connection_string(data_source);

        println!("Connection string: {}", connection_string.color("blue"));

        let pool = PgPool::connect(&connection_string)
            .await
            .map_err(|e| format!("Database connection error: {}", e))?;

        let row = sqlx::query(&limited_query)
            .fetch_optional(&pool)
            .await
            .map_err(|e| format!("Query execution error: {}", e))?;

        pool.close().await;

        Ok(match row {
            Some(row) => Self::row_to_json(row),
            None => Value::Null,
        })
    }

    /// Выполняет несколько запросов в транзакции
    pub async fn execute_batch_queries(
        data_source: &DataSource,
        queries: &[String],
    ) -> Result<Vec<Vec<Value>>, String> {
        let connection_string = Self::build_connection_string(data_source);

        let pool = PgPool::connect(&connection_string)
            .await
            .map_err(|e| format!("Database connection error: {}", e))?;

        let mut transaction = pool
            .begin()
            .await
            .map_err(|e| format!("Transaction start error: {}", e))?;

        let mut results = Vec::new();

        for query in queries {
            let rows = sqlx::query(query)
                .fetch_all(&mut *transaction)
                .await
                .map_err(|e| format!("Query execution error: {}", e))?;

            results.push(rows.into_iter().map(Self::row_to_json).collect());
        }

        transaction
            .commit()
            .await
            .map_err(|e| format!("Transaction commit error: {}", e))?;

        pool.close().await;
        Ok(results)
    }

    /// Тестирует подключение к базе данных
    pub async fn test_connection(data_source: &DataSource) -> Result<bool, String> {
        let connection_string = Self::build_connection_string(data_source);

        let pool = PgPool::connect(&connection_string)
            .await
            .map_err(|e| format!("Database connection error: {}", e))?;

        // Простой тестовый запрос
        let _result = sqlx::query("SELECT 1")
            .fetch_one(&pool)
            .await
            .map_err(|e| format!("Test query error: {}", e))?;

        pool.close().await;
        Ok(true)
    }

    /// Подготавливает запрос с учетом лимита
    fn prepare_query(query: &str, limit: u32) -> String {
        let query_without_semicolon = query.strip_suffix(";").unwrap_or(query);

        format!("{} LIMIT {}", query_without_semicolon, limit)
    }

    /// Строит строку подключения к PostgreSQL
    fn build_connection_string(data_source: &DataSource) -> String {
        format!(
            "postgresql://{}:{}@{}:{}/{}",
            data_source.username,
            data_source.password,
            data_source.host,
            data_source.port,
            data_source.database_name
        )
    }

    /// Преобразует строку базы данных в JSON
    fn row_to_json(row: PgRow) -> Value {
        let mut json_map = serde_json::Map::new();

        for column in row.columns() {
            let column_name = column.name();
            let type_info = column.type_info();

            let value = match type_info.name() {
                "INT2" => {
                    match row.try_get::<Option<i16>, _>(column_name) {
                        Ok(Some(val)) => Value::Number((val as i64).into()),
                        Ok(None) => Value::Null,
                        Err(_) => Value::Null,
                    }
                },
                "INT4" => {
                    match row.try_get::<Option<i32>, _>(column_name) {
                        Ok(Some(val)) => Value::Number((val as i64).into()),
                        Ok(None) => Value::Null,
                        Err(_) => Value::Null,
                    }
                },
                "INT8" => {
                    match row.try_get::<Option<i64>, _>(column_name) {
                        Ok(Some(val)) => Value::Number(val.into()),
                        Ok(None) => Value::Null,
                        Err(_) => Value::Null,
                    }
                },
                "FLOAT4" | "FLOAT8" | "NUMERIC" => {
                    match row.try_get::<Option<f64>, _>(column_name) {
                        Ok(Some(val)) => Value::Number(
                            serde_json::Number::from_f64(val).unwrap_or(serde_json::Number::from(0))
                        ),
                        Ok(None) => Value::Null,
                        Err(_) => Value::Null,
                    }
                },
                "BOOL" => {
                    match row.try_get::<Option<bool>, _>(column_name) {
                        Ok(Some(val)) => Value::Bool(val),
                        Ok(None) => Value::Null,
                        Err(_) => Value::Null,
                    }
                },
                "UUID" => {
                    match row.try_get::<Option<String>, _>(column_name) {
                        Ok(Some(val)) => Value::String(val.to_string()),
                        Ok(None) => Value::Null,
                        Err(_) => Value::Null,
                    }
                },
                "TIMESTAMP" | "TIMESTAMPTZ" => {
                    match row.try_get::<Option<String>, _>(column_name) {
                        Ok(Some(val)) => Value::String(val),
                        Ok(None) => Value::Null,
                        Err(_) => Value::Null,
                    }
                },
                _ => {
                    // Для всех остальных типов пытаемся получить как строку
                    match row.try_get::<Option<String>, _>(column_name) {
                        Ok(Some(val)) => Value::String(val),
                        Ok(None) => Value::Null,
                        Err(_) => Value::Null,
                    }
                }
            };

            json_map.insert(column_name.to_string(), value);
        }

        Value::Object(json_map)
    }
}