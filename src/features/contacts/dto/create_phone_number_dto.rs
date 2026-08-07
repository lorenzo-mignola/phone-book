use sea_orm::ActiveValue::Set;
use serde::Deserialize;

use crate::entity::{country_code::CountryCode, number::Number, phone_numbers};

#[derive(Deserialize)]
pub struct CreatePhoneNumberDto {
    pub country_code: CountryCode,

    pub number: Number,
}

impl From<CreatePhoneNumberDto> for phone_numbers::ActiveModel {
    fn from(create_phone_number_dto: CreatePhoneNumberDto) -> Self {
        phone_numbers::ActiveModel {
            country_code: Set(create_phone_number_dto.country_code),
            number: Set(create_phone_number_dto.number),
            ..Default::default()
        }
    }
}
