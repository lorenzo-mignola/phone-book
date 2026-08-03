use std::fs::create_dir_all;

use sea_orm::{Database, DatabaseConnection, DbErr};

pub async fn connect() -> Result<DatabaseConnection, DbErr> {
    create_dir_all("data").expect("Unable to create the data folder");
    let db = Database::connect("sqlite://data/phone_book.db?mode=rwc").await?;

    db.get_schema_registry("phone_book::entity::*")
        .sync(&db)
        .await?;

    Ok(db)
}
