mod config;
mod controllers;
mod app;
mod requests;
mod responses;

#[tokio::main]
async fn main() {
    // Create the application using the app module
    let app = app::create_app().await;

    // Start the server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Server running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
