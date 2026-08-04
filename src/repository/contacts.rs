use sea_orm::{DatabaseConnection, EntityTrait};

use crate::entity::contacts;
use crate::entity::phone_numbers;
use crate::error::AppError;
use crate::repository::contact_with_numbers::ContactWithNumbers;

pub(crate) async fn find_all(db: &DatabaseConnection) -> Result<Vec<ContactWithNumbers>, AppError> {
    let contact_with_numbers = contacts::Entity::find()
        .find_with_related(phone_numbers::Entity)
        .all(db)
        .await
        .map_err(AppError::Db)?
        .into_iter()
        .map(ContactWithNumbers::from)
        .collect();

    Ok(contact_with_numbers)
}

pub(crate) async fn find_by_id(
    db: &DatabaseConnection,
    id: i32,
) -> Result<ContactWithNumbers, AppError> {
    contacts::Entity::find_by_id(id)
        .find_with_related(phone_numbers::Entity)
        .all(db)
        .await
        .map_err(AppError::Db)?
        .into_iter()
        .next()
        .map(ContactWithNumbers::from)
        .ok_or(AppError::NotFound)
}
