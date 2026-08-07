use sea_orm::ActiveValue::Set;
use serde::Deserialize;

use crate::entity::{contacts, phone_numbers};

use super::CreatePhoneNumberDto;

#[derive(Deserialize)]
pub struct CreateContactDto {
    pub first_name: String,
    pub last_name: Option<String>,
    pub phone_numbers: Vec<CreatePhoneNumberDto>,
}

impl From<CreateContactDto> for (contacts::ActiveModel, Vec<phone_numbers::ActiveModel>) {
    fn from(create_contact_dto: CreateContactDto) -> Self {
        let contact = contacts::ActiveModel {
            first_name: Set(create_contact_dto.first_name),
            last_name: Set(create_contact_dto.last_name),
            ..Default::default()
        };

        let phone_numbers: Vec<phone_numbers::ActiveModel> = create_contact_dto
            .phone_numbers
            .into_iter()
            .map(|number| number.into())
            .collect();

        (contact, phone_numbers)
    }
}
