mod db;
mod routes;
mod handlers;
mod middlewares;
mod models;
mod dto;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let app = routes::make_app()
        .await
        .expect("Failed to create app");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8081").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
