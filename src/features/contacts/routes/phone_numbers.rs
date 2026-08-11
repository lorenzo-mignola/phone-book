use axum::Json;
use axum::extract::{Path, State};

use crate::error::AppError;
use crate::features::contacts::dto::PhoneNumberDto;
use crate::features::contacts::service::phone_numbers;
use crate::state::AppState;

pub(crate) async fn get_all_phone_number_by_contact_id(
    Path(id): Path<i32>,
    State(state): State<AppState>,
) -> Result<Json<Vec<PhoneNumberDto>>, AppError> {
    let phone_numbers = phone_numbers::find_phone_numbers_by_contact_id(id, &state).await?;

    Ok(Json(phone_numbers))
}
