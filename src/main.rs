use phone_book::{routes, state::build_state};
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    dotenvy::dotenv().ok();

    let state = build_state().await?;

    let app = routes::router(state);

    let address = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(address).await?;

    info!("🚀 Server starting at {}", address);

    axum::serve(listener, app).await?;

    Ok(())
}
