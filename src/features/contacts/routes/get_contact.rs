use axum::{
    Json,
    extract::{Path, State},
};

use crate::{error::AppError, state::AppState};

use crate::features::contacts::dto::ContactDto;
use crate::features::contacts::repository;

pub(crate) async fn get_contact(
    Path(id): Path<i32>,
    State(state): State<AppState>,
) -> Result<Json<ContactDto>, AppError> {
    let contact = repository::contacts::find_by_id(&state.db, id).await?;

    Ok(Json(contact.into()))
}
