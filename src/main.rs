use phone_book::{db, routes, state};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let connection_string = db::get_connection_string()?;
    let db_connection = db::connect(connection_string).await?;

    let app = routes::router(state::AppState { db: db_connection });

    let address = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(address).await?;

    println!("🚀 Server starting at {}", address);
    axum::serve(listener, app).await?;

    Ok(())
}
