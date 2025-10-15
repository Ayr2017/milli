// Импортируем необходимые модули из крейта rusqlite
use rusqlite::{Connection, Result, params};
// Импортируем макрос для обработки ошибок
use anyhow::{Context, anyhow};
use chrono::DateTime;
use r2d2_sqlite::SqliteConnectionManager;
use r2d2::Pool;
use crate::db::user::User;




type DbPool = Pool<SqliteConnectionManager>;

/// Основная структура для работы с базой данных
/// Инкапсулирует все операции с SQLite
#[derive(Debug, Clone)]
pub struct Database {
    pub pool: DbPool,
}

impl Database {
    pub(crate) fn new(database_path: &str) -> Result<Self, anyhow::Error> {
        let manager = SqliteConnectionManager::file(database_path);
        let pool = Pool::new(manager)?;
        let db = Database { pool };
        db.init_tables()?;
        Ok(db)
    }

    /// Инициализирует необходимые таблицы в базе данных
    /// Создает таблицы, если они еще не существуют
    fn init_tables(&self) -> Result<(), anyhow::Error> {
        let conn = self.pool.get()?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY, name TEXT, email TEXT)",
            [],
        )?;

        conn.execute(
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
                );",
            [],
        )?;
        Ok(())
    }
    pub async fn get_pool_connection(&self) -> Result<r2d2::PooledConnection<SqliteConnectionManager>, anyhow::Error> {
        Ok(self.pool.get()?)
    }
}