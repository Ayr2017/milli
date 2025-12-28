use clap::Parser;
use std::ops::Deref;
use anyhow::Context;
use crate::config::application::ApplicationConfig;
use crate::database::Database;
use crate::state::AppState;
use crate::presentation::cli::cli_app::Args;

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


mod infrastructure;
mod utilits;
mod modules;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = <Args as clap::Parser>::parse();
    let config = ApplicationConfig::new().await.expect("Failed to load config");
    let database = Database::new(&config.db_path).await?;
    let state = AppState::new(config, database).await?;

    // Инициализация логирования
    tracing_subscriber::fmt::init();
    tracing::info!("Сообщение");
    
    if args.command.is_some() {
        args.execute(state).await?;
        return Ok(()); // Завершаем выполнение, не запуская сервер
    }

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
