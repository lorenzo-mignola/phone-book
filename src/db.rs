use sea_orm::{Database, DatabaseConnection, DbErr};

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
