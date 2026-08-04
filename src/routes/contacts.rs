use axum::{
    Json, Router,
    extract::{Path, State},
    routing::get,
};

use crate::{dto::contact_dto::ContactDto, error::AppError, repository, state::AppState};

pub(super) fn contacts_router() -> Router<AppState> {
    Router::new()
        .route("/contacts", get(list_contacts))
        .route("/contacts/{id}", get(get_contact))
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

    Ok(Json(ContactDto::from(contact)))
}
