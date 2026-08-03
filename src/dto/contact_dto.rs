use serde::Serialize;

use crate::entity::contacts;

#[derive(Serialize)]
pub struct ContactDto {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
}

impl From<contacts::Model> for ContactDto {
    fn from(model: contacts::Model) -> Self {
        Self {
            id: model.id,
            first_name: model.first_name,
            last_name: model.last_name,
        }
    }
}
