use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::shared::types::sex::Sex;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Dogs {
    pub id: Uuid,
    pub name: String,
    pub birthdate: Option<DateTime<Utc>>,
    pub races: Vec<String>,
    pub img_url: Option<String>,
    pub sex: Sex,
}
