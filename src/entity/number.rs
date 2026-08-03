use sea_orm::DeriveValueType;

#[derive(Clone, Debug, PartialEq, Eq, DeriveValueType)]
pub struct Number(String);
