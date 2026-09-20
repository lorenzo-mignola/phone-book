use std::env;

use sea_orm::DatabaseConnection;

use crate::{db, state};

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub dicebar_url: String,
}

pub async fn build_state() -> Result<AppState, Box<dyn std::error::Error>> {
    let connection_string = env::var("DATABASE_URL")?;
    let dicebar_url = env::var("DICEBAR_URL")?;
    let db_connection = db::connect(connection_string).await?;

    Ok(state::AppState {
        db: db_connection,
        dicebar_url,
    })
}
