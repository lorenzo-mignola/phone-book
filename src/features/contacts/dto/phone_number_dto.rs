use serde::Serialize;

use crate::entity::phone_numbers;

#[derive(Serialize)]
pub struct PhoneNumberDto(String);

impl From<phone_numbers::Model> for PhoneNumberDto {
    fn from(model: phone_numbers::Model) -> Self {
        let number = format!("{} {}", model.country_code.prefix(), model.number);

        Self(number)
    }
}
