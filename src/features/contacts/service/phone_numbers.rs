use crate::error::AppError;
use crate::features::contacts::dto::PhoneNumberDto;
use crate::features::contacts::repository::phone_numbers;
use crate::state::AppState;

pub(crate) async fn find_phone_numbers_by_contact_id(
    id: i32,
    state: &AppState,
) -> Result<Vec<PhoneNumberDto>, AppError> {
    let phone_numbers = phone_numbers::find_phone_numbers_by_contact_id(&state.db, id).await?;

    Ok(phone_numbers
        .into_iter()
        .map(PhoneNumberDto::from)
        .collect())
}
