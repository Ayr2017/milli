use std::ops::Deref;
use std::sync::Arc;
use crate::config::application::ApplicationConfig;
use crate::database::Database;
use crate::state::AppState;
use r2d2_sqlite::SqliteConnectionManager;
use r2d2::Pool;

mod config;
mod controllers;
mod app;
mod requests;
mod responses;
mod state;
mod database;

#[tokio::main]
async fn main() {
    let config = ApplicationConfig::new().unwrap();
    println!("Путь к базе данных: {}", config.db_path); // Добавьте эту строку
    type DbPool = Pool<SqliteConnectionManager>;


    let state = AppState::new(config.clone());

    let db = Database::new(config.db_path.deref()).unwrap();
    println!("База данных успешно инициализирована по пути: {}", config.db_path);

    println!("База данных успешно инициализирована");
    // let user1_id = db.add_user("Алексей Петров", "alexey@example.com").unwrap();
    // let user1_name = db.add_user("Николай Петров", "nick@example.com").unwrap();
    let users = db.get_all_users().unwrap();
    for user in users {
        println!("ID пользователя: {}, Имя: {}", user.id, user.name);
    }



    // Create the application using the app module
    let app = app::create_app(state).await;

    // Start the server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Server running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
