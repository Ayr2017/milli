use std::sync::Arc;
use crate::config::application::ApplicationConfig;
use crate::state::AppState;

mod config;
mod controllers;
mod app;
mod requests;
mod responses;
mod state;

#[tokio::main]
async fn main() {
    let state = AppState::new(ApplicationConfig::new().expect("Error loading config"));
    // Create the application using the app module
    let app = app::create_app(state).await;

    // Start the server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Server running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
