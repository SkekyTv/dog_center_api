use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq, Eq)]
#[sqlx(type_name = "sex")]
pub enum Sex {
    F,
    M,
    #[sqlx(rename = "Other")]
    Other,
}
