use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
};

use crate::{
    dto::{contact_dto::ContactDto, create_contact_dto::CreateContactDto},
    error::AppError,
    repository::{self},
    state::AppState,
};

pub(super) fn contacts_router() -> Router<AppState> {
    Router::new()
        .route("/contacts", get(list_contacts))
        .route("/contacts/{id}", get(get_contact))
        .route("/contacts", post(save_contact))
}

async fn list_contacts(State(state): State<AppState>) -> Result<Json<Vec<ContactDto>>, AppError> {
    let contacts = repository::contacts::find_all(&state.db).await?;

    let contacts_dto = contacts.into_iter().map(ContactDto::from).collect();
    Ok(Json(contacts_dto))
}

async fn get_contact(
    Path(id): Path<i32>,
    State(state): State<AppState>,
) -> Result<Json<ContactDto>, AppError> {
    let contact = repository::contacts::find_by_id(&state.db, id).await?;

    Ok(Json(contact.into()))
}

pub(super) async fn save_contact(
    State(state): State<AppState>,
    Json(create_contact): Json<CreateContactDto>,
) -> Result<(StatusCode, Json<ContactDto>), AppError> {
    let (contact, phone_numbers) = create_contact.into();

    let created_contact =
        repository::contacts::create_contact(&state.db, contact, phone_numbers).await?;

    Ok((StatusCode::CREATED, Json(created_contact.into())))
}
