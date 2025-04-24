use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
#[sqlx(type_name = "sex", rename_all = "UPPERCASE")]
pub enum Sex {
    F,
    M,
    Other,
}
