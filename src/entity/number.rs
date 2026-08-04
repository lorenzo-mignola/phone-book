use std::fmt::Display;

use sea_orm::DeriveValueType;

#[derive(Clone, Debug, PartialEq, Eq, DeriveValueType)]
pub struct Number(pub String);

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}
