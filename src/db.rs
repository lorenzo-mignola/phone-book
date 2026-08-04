use std::{env, error::Error, fs::create_dir_all};

use sea_orm::{Database, DatabaseConnection, DbErr};

pub fn get_connection_string() -> Result<String, Box<dyn Error>> {
    let connection_string = env::var("DATABASE_URL")?;

    create_dir_all("data")?;

    Ok(connection_string)
}

pub async fn setup_schema(db: &DatabaseConnection) -> Result<(), DbErr> {
    db.get_schema_registry("phone_book::entity::*")
        .sync(db)
        .await
}

pub async fn connect(connection_string: String) -> Result<DatabaseConnection, DbErr> {
    let db = Database::connect(connection_string).await?;

    setup_schema(&db).await?;

    Ok(db)
}
