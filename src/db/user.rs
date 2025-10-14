use anyhow::Context;
use rusqlite::{params, Connection};

/// Структура для представления пользователя в базе данных
/// Это Rust-представление строки из таблицы users
#[derive(Debug)]
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
    pub fn store(&self, conn: Connection) -> rusqlite::Result<i32, anyhow::Error> {
        // let conn = self.pool.get()?;

        // SQL-запрос для вставки нового пользователя
        // ?1, ?2 - параметры, которые будут подставлены из аргументов функции
        let sql = "INSERT INTO users (name, email) VALUES (?1, ?2)";

        // Выполняем запрос на вставку
        // params![] макрос для передачи параметров в запрос
        conn.execute(sql, params![self.name, self.email])
            .context("Не удалось добавить пользователя".to_string())?;

        // Получаем ID последней вставленной записи
        // last_insert_rowid() - функция SQLite, возвращающая ID последней вставленной строки
        let user_id = conn.last_insert_rowid() as i32;

        println!("Пользователь '{}' добавлен с ID: {}", self.name, user_id);

        Ok(user_id)
    }

    pub fn get_all_users(&self, conn: Connection) -> rusqlite::Result<Vec<User>, anyhow::Error> {
        // let conn = self.pool.get()?;

        // SQL-запрос для выборки всех пользователей
        let sql = "SELECT id, name, email FROM users ORDER BY id";

        // Подготавливаем запрос (компилируем SQL в байткод SQLite)
        let mut statement = conn.prepare(sql)
            .context("Не удалось подготовить запрос для получения пользователей".to_string())?;

        // Выполняем запрос и преобразуем результаты в итератор
        // query_map() выполняет запрос и преобразует каждую строку в указанный тип
        let user_iter = statement.query_map([], |row| {
            // Анонимная функция, которая преобразует строку результата в структуру User
            Ok(User {
                id: row.get(0)?,      // Получаем значение из колонки с индексом 0 (id)
                name: row.get(1)?,    // Получаем значение из колонки с индексом 1 (name)
                email: row.get(2)?,   // Получаем значение из колонки с индексом 2 (email)
            })
        })
            .context("Не удалось выполнить запрос получения пользователей".to_string())?;

        // Собираем все результаты в вектор
        let mut users = Vec::new();
        for user in user_iter {
            // Обрабатываем каждую строку, преобразуя Result в anyhow::Error при необходимости
            users.push(user.context("Ошибка при обработке строки результата".to_string())?);
        }

        Ok(users)
    }

    /// Удаляет пользователя по ID
    /// # Arguments
    /// * `user_id` - ID пользователя для удаления
    /// # Returns
    /// * `Result<bool>` - true если пользователь был удален, false если не найден, или ошибка
    pub fn delete_user(&self, conn: Connection, user_id: i32) -> rusqlite::Result<bool, anyhow::Error> {
        // let conn = self.pool.get()?;
        // SQL-запрос для удаления пользователя по ID
        let sql = "DELETE FROM users WHERE id = ?1";

        // Выполняем запрос удаления
        // execute возвращает количество затронутых строк
        let rows_affected = conn.execute(sql, params![user_id])
            .context("Не удалось выполнить удаление пользователя".to_string())?;

        // Если затронута хотя бы одна строка - пользователь был удален
        let deleted = rows_affected > 0;

        if deleted {
            println!("Пользователь с ID {} удален", user_id);
        } else {
            println!("Пользователь с ID {} не найден", user_id);
        }

        Ok(deleted)
    }
}