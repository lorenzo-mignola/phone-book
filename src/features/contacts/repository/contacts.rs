use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::Set;
use sea_orm::ConnectionTrait;
use sea_orm::EntityTrait;
use sea_orm::TransactionSession;
use sea_orm::TransactionTrait;

use crate::entity::contacts;
use crate::entity::phone_numbers;
use crate::error::AppError;

use super::contact_with_numbers::ContactWithNumbers;

pub(crate) async fn find_all(
    db: &(impl ConnectionTrait + TransactionTrait),
) -> Result<Vec<ContactWithNumbers>, AppError> {
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
    db: &(impl ConnectionTrait + TransactionTrait),
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

pub(crate) async fn create_contact(
    db: &(impl ConnectionTrait + TransactionTrait),
    contact: contacts::ActiveModel,
    phone_numbers: Vec<phone_numbers::ActiveModel>,
) -> Result<ContactWithNumbers, AppError> {
    let txn = db.begin().await.map_err(AppError::Db)?;

    let saved_contact = contact.insert(&txn).await.map_err(AppError::Db)?;

    let phone_numbers: Vec<phone_numbers::ActiveModel> = phone_numbers
        .into_iter()
        .map(|number| phone_numbers::ActiveModel {
            contact_id: Set(saved_contact.id),
            ..number
        })
        .collect();

    for phone_number in phone_numbers {
        phone_number.insert(&txn).await.map_err(AppError::Db)?;
    }

    txn.commit().await.map_err(AppError::Db)?;

    find_by_id(db, saved_contact.id).await
}
