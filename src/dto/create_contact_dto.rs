use sea_orm::ActiveValue::Set;
use serde::Deserialize;

use crate::{dto::create_phone_number_dto::CreatePhoneNumberDto, entity::contacts};

#[derive(Deserialize)]
pub struct CreateContactDto {
    pub first_name: String,
    pub last_name: Option<String>,
    pub phone_numbers: Vec<CreatePhoneNumberDto>,
}

impl From<CreateContactDto> for contacts::ActiveModel {
    fn from(create_contact_dto: CreateContactDto) -> Self {
        contacts::ActiveModel {
            first_name: Set(create_contact_dto.first_name),
            last_name: Set(create_contact_dto.last_name),
            ..Default::default()
        }
    }
}
