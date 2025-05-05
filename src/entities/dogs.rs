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
    pub img_url: Option<String>, // unimplemented!()
    pub sex: Sex,
}

impl Dogs {
    pub fn new(
        name: String,
        sex: Sex,
        birthdate: Option<DateTime<Utc>>,
        races: Vec<String>,
    ) -> Self {
        Dogs {
            id: Uuid::new_v4(),
            name,
            birthdate,
            races,
            sex,
            img_url: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dog_constructor_minimal_params() {
        let dog = Dogs::new("pupuce".to_string(), Sex::F, None, [].to_vec());
        assert_eq!(dog.name, "pupuce");
        assert_eq!(dog.sex, Sex::F);
    }
}
