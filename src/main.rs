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
mod db;

#[tokio::main]
async fn main() {
    let config = ApplicationConfig::new().unwrap();
    println!("Путь к базе данных: {}", config.db_path); // Добавьте эту строку

    let mut state = AppState::new(config.clone());

    let db = Database::new(config.db_path.deref()).unwrap();
    state.set_database(Arc::new(db.clone()));

    // Create the application using the app module
    let app = app::create_app(state).await;

    // Start the server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Server running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
