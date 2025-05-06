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
    pub weight: Option<i32>,
    pub img_url: Option<String>, // unimplemented!()
    pub sex: Sex,
    pub icad_id: Option<String>,
}

impl Dogs {
    pub fn new(
        name: String,
        sex: Sex,
        birthdate: Option<DateTime<Utc>>,
        races: Vec<String>,
        weight: Option<i32>,
        icad_id: Option<String>,
    ) -> Self {
        Dogs {
            id: Uuid::new_v4(),
            name,
            birthdate,
            races,
            sex,
            img_url: None,
            weight,
            icad_id,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dog_constructor_minimal_params() {
        let dog = Dogs::new(
            "pupuce".to_string(),
            Sex::F,
            None,
            [].to_vec(),
            Some(100),
            None,
        );
        assert_eq!(dog.name, "pupuce");
        assert_eq!(dog.sex, Sex::F);
        assert_eq!(dog.weight, Some(100));
    }
}
