use sea_orm::DeriveActiveEnum;

use sea_orm::entity::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum CountryCode {
    #[sea_orm(string_value = "CH")]
    CH,

    #[sea_orm(string_value = "IT")]
    IT,
}

impl CountryCode {
    pub fn prefix(&self) -> &'static str {
        match self {
            CountryCode::CH => "+41",
            CountryCode::IT => "+39",
        }
    }
}
