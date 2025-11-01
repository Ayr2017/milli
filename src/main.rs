use std::ops::Deref;
use anyhow::Context;
use crate::config::application::ApplicationConfig;
use crate::database::Database;
use crate::state::AppState;

mod config;
mod controllers;
mod app;
mod requests;
mod responses;
mod state;
mod database;
mod db;
mod services;
mod models;
mod repositories;
mod application;
mod presentation;
mod domain;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Инициализация логирования
    tracing_subscriber::fmt::init();

    initialize_and_run_server().await
}

async fn initialize_and_run_server() -> Result<(), Box<dyn std::error::Error>> {
    let config = ApplicationConfig::new().await.expect("Failed to load config");
    let database = Database::new(&config.db_path).await?;
    let state = AppState::new(config, database).await?;
    let app = app::create_app(state).await;

    let server_address = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(server_address).await?;
    println!("🚀 Server running on http://{}", server_address);

    axum::serve(listener, app).await?;
    Ok(())
}
