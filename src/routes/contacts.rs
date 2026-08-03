use axum::{Json, Router, extract::State, routing::get};

use crate::{dto::contact_dto::ContactDto, error::AppError, repository, state::AppState};

pub(super) fn contacts_router() -> Router<AppState> {
    Router::new().route("/contacts", get(list_contacts))
}

async fn list_contacts(State(state): State<AppState>) -> Result<Json<Vec<ContactDto>>, AppError> {
    let contacts = repository::contacts::find_all(&state.db).await?;

    let contacts_dto = contacts.into_iter().map(ContactDto::from).collect();
    Ok(Json(contacts_dto))
}
