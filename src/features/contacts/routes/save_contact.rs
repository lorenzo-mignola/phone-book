use axum::{Json, extract::State, http::StatusCode};

use crate::{error::AppError, state::AppState};

use crate::features::contacts::dto::{ContactDto, CreateContactDto};
use crate::features::contacts::repository;

pub(crate) async fn save_contact(
    State(state): State<AppState>,
    Json(create_contact): Json<CreateContactDto>,
) -> Result<(StatusCode, Json<ContactDto>), AppError> {
    let (contact, phone_numbers) = create_contact.into();

    let created_contact =
        repository::contacts::create_contact(&state.db, contact, phone_numbers).await?;

    Ok((StatusCode::CREATED, Json(created_contact.into())))
}
