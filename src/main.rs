use std::env;

use phone_book::{db, routes, state};
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let connection_string = env::var("DATABASE_URL")?;
    let db_connection = db::connect(connection_string).await?;

    let app = routes::router(state::AppState { db: db_connection });

    let address = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(address).await?;

    info!("🚀 Server starting at {}", address);

    axum::serve(listener, app).await?;

    Ok(())
}
