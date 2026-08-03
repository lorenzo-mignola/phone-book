use sea_orm::{DatabaseConnection, EntityTrait};

use crate::entity::contacts;
use crate::error::AppError;

pub async fn find_all(db: &DatabaseConnection) -> Result<Vec<contacts::Model>, AppError> {
    contacts::Entity::find().all(db).await.map_err(AppError::Db)
}
