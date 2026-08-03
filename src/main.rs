mod db;
mod dto;
mod entity;
mod error;
mod repository;
mod routes;
mod state;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_connection = db::connect().await?;
    let app = routes::router(state::AppState { db: db_connection });

    let address = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(address).await?;

    println!("🚀 Server starting at {}", address);
    axum::serve(listener, app).await?;

    Ok(())
}
