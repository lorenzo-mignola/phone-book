use serde::Serialize;

use crate::features::contacts::repository::ContactWithNumbers;

#[derive(Serialize)]
pub struct ContactDto {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub phone_numbers: Vec<super::PhoneNumberDto>,
}

impl From<ContactWithNumbers> for ContactDto {
    fn from(model: ContactWithNumbers) -> Self {
        let ContactWithNumbers { contact, numbers } = model;
        Self {
            id: contact.id,
            first_name: contact.first_name,
            last_name: contact.last_name.unwrap_or_default(),
            phone_numbers: numbers
                .into_iter()
                .map(super::PhoneNumberDto::from)
                .collect(),
        }
    }
}
