use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, TransactionTrait};

use crate::{entity::phone_numbers, error::AppError};

pub(crate) async fn find_phone_numbers_by_contact_id(
    db: &(impl ConnectionTrait + TransactionTrait),
    id: i32,
) -> Result<Vec<phone_numbers::Model>, AppError> {
    let phone_numbers = phone_numbers::Entity::find()
        .filter(phone_numbers::Column::ContactId.eq(id))
        .all(db)
        .await
        .map_err(AppError::Db)?;

    Ok(phone_numbers)
}
