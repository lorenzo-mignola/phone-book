use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "phone_numbers")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,

    pub country_code: super::country_code::CountryCode,

    pub number: super::number::Number,

    pub contact_id: i32,

    #[sea_orm(belongs_to, from = "contact_id", to = "id")]
    pub contact: BelongsTo<super::contacts::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
