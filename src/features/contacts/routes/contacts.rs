use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use crate::{error::AppError, state::AppState};

use crate::features::contacts::dto::{ContactDto, CreateContactDto};
use crate::features::contacts::service;

pub(crate) async fn get_contact(
    Path(id): Path<i32>,
    State(state): State<AppState>,
) -> Result<Json<ContactDto>, AppError> {
    let contact = service::contacts::find_by_id(id, &state).await?;

    Ok(Json(contact))
}

pub(crate) async fn list_contacts(
    State(state): State<AppState>,
) -> Result<Json<Vec<ContactDto>>, AppError> {
    let contacts_dto = service::contacts::find_all(&state).await?;

    Ok(Json(contacts_dto))
}

pub(crate) async fn save_contact(
    State(state): State<AppState>,
    Json(create_contact): Json<CreateContactDto>,
) -> Result<(StatusCode, Json<ContactDto>), AppError> {
    let created_contact = service::contacts::create_contact(create_contact, &state).await?;

    Ok((StatusCode::CREATED, Json(created_contact)))
}

pub(crate) async fn update_contact(
    Path(id): Path<i32>,
    State(state): State<AppState>,
    Json(contact_to_update): Json<CreateContactDto>,
) -> Result<Json<ContactDto>, AppError> {
    let contact_updated = service::contacts::update_contact(id, contact_to_update, &state).await?;

    Ok(Json(contact_updated))
}
