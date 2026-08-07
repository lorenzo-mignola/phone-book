use axum::{Json, extract::State};

use crate::{error::AppError, state::AppState};

use crate::features::contacts::dto::ContactDto;
use crate::features::contacts::repository;

pub(crate) async fn list_contacts(
    State(state): State<AppState>,
) -> Result<Json<Vec<ContactDto>>, AppError> {
    let contacts = repository::contacts::find_all(&state.db).await?;

    let contacts_dto = contacts.into_iter().map(ContactDto::from).collect();
    Ok(Json(contacts_dto))
}
