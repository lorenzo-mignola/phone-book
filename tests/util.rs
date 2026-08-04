use axum::Router;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

async fn connect_in_memory() -> DatabaseConnection {
    let mut opts = ConnectOptions::new("sqlite::memory:");
    opts.max_connections(1);
    let db = Database::connect(opts)
        .await
        .expect("Error getting db connection");

    phone_book::db::setup_schema(&db)
        .await
        .expect("Error setup schema");

    db
}

pub async fn setup_test() -> Router {
    let db = connect_in_memory().await;
    phone_book::routes::router(phone_book::state::AppState { db })
}
