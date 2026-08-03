use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "contacts")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,

    pub first_name: String,

    pub last_name: String,

    #[sea_orm(has_many)]
    pub phone_numbers: HasMany<super::phone_numbers::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
