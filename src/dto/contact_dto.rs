use serde::Serialize;

use crate::{
    dto::phone_number_dto::PhoneNumberDto, repository::contact_with_numbers::ContactWithNumbers,
};

#[derive(Serialize)]
pub struct ContactDto {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub phone_numbers: Vec<PhoneNumberDto>,
}

impl From<ContactWithNumbers> for ContactDto {
    fn from(model: ContactWithNumbers) -> Self {
        let ContactWithNumbers { contact, numbers } = model;
        Self {
            id: contact.id,
            first_name: contact.first_name,
            last_name: contact.last_name.unwrap_or_default(),
            phone_numbers: numbers.into_iter().map(PhoneNumberDto::from).collect(),
        }
    }
}
