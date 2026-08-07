use crate::entity::{contacts, phone_numbers};

pub struct ContactWithNumbers {
    pub contact: contacts::Model,
    pub numbers: Vec<phone_numbers::Model>,
}

impl From<(contacts::Model, Vec<phone_numbers::Model>)> for ContactWithNumbers {
    fn from(model: (contacts::Model, Vec<phone_numbers::Model>)) -> ContactWithNumbers {
        let (contact, numbers) = model;
        ContactWithNumbers { contact, numbers }
    }
}
