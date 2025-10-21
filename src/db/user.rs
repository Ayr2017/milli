use anyhow::{Context, Result};
use sqlx::{Pool, Sqlite, query, query_as, FromRow};

/// Структура для представления пользователя в базе данных
/// Это Rust-представление строки из таблицы users
#[derive(Debug, FromRow)]
pub struct User {
    pub id: i32,        // INTEGER PRIMARY KEY
    pub name: String,   // TEXT NOT NULL
    pub email: String,  // TEXT UNIQUE
}

impl User {
    /// Добавляет нового пользователя в базу данных
    /// # Arguments
    /// * `name` - имя пользователя
    /// * `email` - email пользователя (должен быть уникальным)
    /// # Returns
    /// * `Result<i32>` - ID созданного пользователя или ошибка
    pub async fn store(&self, pool: &Pool<Sqlite>) -> Result<i32, anyhow::Error> {
        // SQL-запрос для вставки нового пользователя
        let sql = "INSERT INTO users (name, email) VALUES (?, ?)";

        // Выполняем запрос на вставку
        let result = query(sql)
            .bind(&self.name)
            .bind(&self.email)
            .execute(pool)
            .await
            .context("Не удалось добавить пользователя".to_string())?;

        // Получаем ID последней вставленной записи
        let user_id = result.last_insert_rowid() as i32;

        println!("Пользователь '{}' добавлен с ID: {}", self.name, user_id);

        Ok(user_id)
    }

    pub async fn get_all_users(&self, pool: &Pool<Sqlite>) -> Result<Vec<User>, anyhow::Error> {
        // SQL-запрос для выборки всех пользователей
        let sql = "SELECT id, name, email FROM users ORDER BY id";

        // Выполняем запрос и получаем результаты
        let users = query_as::<_, User>(sql)
            .fetch_all(pool)
            .await
            .context("Не удалось выполнить запрос получения пользователей".to_string())?;

        Ok(users)
    }

    /// Удаляет пользователя по ID
    /// # Arguments
    /// * `user_id` - ID пользователя для удаления
    /// # Returns
    /// * `Result<bool>` - true если пользователь был удален, false если не найден, или ошибка
    pub async fn delete_user(&self, pool: &Pool<Sqlite>, user_id: i32) -> Result<bool, anyhow::Error> {
        // SQL-запрос для удаления пользователя по ID
        let sql = "DELETE FROM users WHERE id = ?";

        // Выполняем запрос удаления
        let result = query(sql)
            .bind(user_id)
            .execute(pool)
            .await
            .context("Не удалось выполнить удаление пользователя".to_string())?;

        // Если затронута хотя бы одна строка - пользователь был удален
        let deleted = result.rows_affected() > 0;

        if deleted {
            println!("Пользователь с ID {} удален", user_id);
        } else {
            println!("Пользователь с ID {} не найден", user_id);
        }

        Ok(deleted)
    }
}
