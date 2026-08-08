use crate::error::AppError;
use crate::features::contacts::dto::{ContactDto, CreateContactDto};

use crate::features::contacts::repository;
use crate::state::AppState;

pub(crate) async fn find_all(state: &AppState) -> Result<Vec<ContactDto>, AppError> {
    let contacts = repository::contacts::find_all(&state.db).await?;

    Ok(contacts.into_iter().map(ContactDto::from).collect())
}

pub(crate) async fn find_by_id(id: i32, state: &AppState) -> Result<ContactDto, AppError> {
    let contact = repository::contacts::find_by_id(&state.db, id).await?;

    Ok(contact.into())
}

pub(crate) async fn create_contact(
    create_contact_dto: CreateContactDto,
    state: &AppState,
) -> Result<ContactDto, AppError> {
    let (contact, phone_numbers) = create_contact_dto.into();

    let created_contact =
        repository::contacts::create_contact(&state.db, contact, phone_numbers).await?;

    Ok(created_contact.into())
}

pub(crate) async fn update_contact(
    id: i32,
    contact_to_update: CreateContactDto,
    state: &AppState,
) -> Result<ContactDto, AppError> {
    let (contact_to_update, phone_numbers_to_update) = contact_to_update.into();

    let udpated_contact = repository::contacts::update_contact(
        &state.db,
        id,
        contact_to_update,
        phone_numbers_to_update,
    )
    .await?;

    Ok(udpated_contact.into())
}
