use sea_orm::{DatabaseConnection, EntityTrait};

use crate::entity::contacts;
use crate::entity::phone_numbers;
use crate::error::AppError;
use crate::repository::contact_with_numbers::ContactWithNumbers;

pub async fn find_all(db: &DatabaseConnection) -> Result<Vec<ContactWithNumbers>, AppError> {
    let contacts = contacts::Entity::find()
        .find_with_related(phone_numbers::Entity)
        .all(db)
        .await
        .map_err(AppError::Db)?;

    Ok(contacts.into_iter().map(ContactWithNumbers::from).collect())
}
