use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Trainer {
    pub id: Uuid,
    pub email: String,
    pub pdw: String,
    pub name: String,
    pub phone_number: Option<String>,
    pub birthdate: Option<String>,
    pub sex: Sex,
    pub img_url: Option<String>,
    pub created_at: DateTime<Utc>,
}
