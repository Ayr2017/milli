use sqlx::{sqlite::{SqlitePool, SqlitePoolOptions}, Sqlite, Pool, query};
use anyhow::Result;

/// Основная структура для работы с базой данных
/// Инкапсулирует все операции с SQLite
#[derive(Debug, Clone)]
pub struct Database {
    pub pool: Pool<Sqlite>,
}

impl Database {
    pub(crate) async fn new(database_path: &str) -> Result<Self, anyhow::Error> {
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&format!("sqlite:{}", database_path))
            .await?;

        let db = Database { pool };
        db.init_tables().await?;
        Ok(db)
    }

    /// Инициализирует необходимые таблицы в базе данных
    /// Создает таблицы, если они еще не существуют
    async fn init_tables(&self) -> Result<(), anyhow::Error> {
        // Create users table
        query(
            "CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY, name TEXT, email TEXT)"
        )
        .execute(&self.pool)
        .await?;
        

        // Create data_sources table
        query(
            "CREATE TABLE IF NOT EXISTS data_sources (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                host TEXT NOT NULL,
                database TEXT NOT NULL,
                username TEXT NOT NULL,
                password TEXT NOT NULL,
                port INTEGER NOT NULL CHECK(port > 0 AND port <= 65535),
                database_path TEXT NOT NULL,
                database_name TEXT NOT NULL,
                database_type TEXT NOT NULL CHECK(database_type IN ('sqlite', 'mysql', 'postgresql')),
                created_at TEXT NOT NULL DEFAULT (datetime('now')),
                updated_at TEXT NOT NULL DEFAULT (datetime('now'))
            );"
        )
        .execute(&self.pool)
        .await?;

        query(
            "CREATE TABLE IF NOT EXISTS index_data_queries (id INTEGER PRIMARY KEY, data_source_id int, index_uid TEXT, query TEXT, created_at TEXT NOT NULL DEFAULT (datetime('now')), updated_at TEXT NOT NULL DEFAULT (datetime('now')))"
        )
            .execute(&self.pool)
            .await?;

        let migration_sql = include_str!("./modules/queue/storage/migrations/001_initial.sql");
        let mut transaction = self.pool.begin().await?;

        sqlx::query(migration_sql)
            .execute(&mut *transaction)
            .await?;

        transaction.commit().await?;

        Ok(())
    }

    pub async fn get_pool_connection(&self) -> Result<&Pool<Sqlite>, anyhow::Error> {
        Ok(&self.pool)
    }
}
