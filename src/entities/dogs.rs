use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::shared::types::sex::Sex;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Dog {
    pub id: Uuid,
    pub name: String,
    pub birthdate: Option<DateTime<Utc>>,
    pub races: Vec<String>,
    pub weight: Option<i32>,
    pub img_url: Option<String>, // unimplemented!()
    pub sex: Sex,
    pub icad_id: Option<String>,
}

impl Dog {
    pub fn new(
        name: String,
        sex: Sex,
        birthdate: Option<DateTime<Utc>>,
        races: Vec<String>,
        weight: Option<i32>,
        icad_id: Option<String>,
    ) -> Self {
        Dog {
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

    pub fn update(
        &self,
        name: Option<String>,
        sex: Option<Sex>,
        birthdate: Option<Option<DateTime<Utc>>>,
        races: Option<Vec<String>>,
        weight: Option<Option<i32>>,
        icad_id: Option<Option<String>>,
    ) -> Self {
        Dog {
            id: self.id,
            name: name.unwrap_or_else(|| self.name.clone()),
            sex: sex.unwrap_or(self.sex),
            birthdate: match birthdate {
                Some(inner) => inner,
                None => self.birthdate,
            },
            races: races.unwrap_or_else(|| self.races.clone()),
            weight: match weight {
                Some(inner) => inner,
                None => self.weight,
            },
            img_url: self.img_url.clone(),
            icad_id: match icad_id {
                Some(inner) => inner,
                None => self.icad_id.clone(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dog_constructor_minimal_params() {
        let dog = Dog::new(
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

    #[test]
    fn test_update_dog_partial_update() {
        let original = Dog::new(
            "Rex".to_string(),
            Sex::M,
            Some(Utc::now()),
            vec!["Labrador".to_string()],
            Some(25),
            Some("ICAD123".to_string()),
        );

        let updated = original.update(
            Some("Max".to_string()),
            Some(Sex::F),
            None,
            Some(vec!["Caniche".to_string()]),
            None,
            None,
        );

        assert_eq!(updated.name, "Max");
        assert_eq!(updated.sex, Sex::F);
        assert_eq!(updated.races, vec!["Caniche"]);
        assert_eq!(updated.birthdate, original.birthdate); // inchangé
        assert_eq!(updated.weight, original.weight); // inchangé
        assert_eq!(updated.icad_id, original.icad_id); // inchangé
    }

    #[test]
    fn test_update_dog_remove_optional_field() {
        let original = Dog::new(
            "Pupuce".to_string(),
            Sex::F,
            None,
            vec!["Shiba".to_string()],
            Some(10),
            Some("ICAD456".to_string()),
        );

        let updated = original.update(
            None,
            None,
            None,
            None,
            Some(None), // on retire le poids
            Some(None), // on retire l'ICAD
        );

        assert_eq!(updated.name, original.name);
        assert_eq!(updated.weight, None);
        assert_eq!(updated.icad_id, None);
    }

    #[test]
    fn test_update_dog_no_changes() {
        let original = Dog::new(
            "Rex".to_string(),
            Sex::M,
            None,
            vec!["Labrador".to_string()],
            Some(30),
            None,
        );

        let updated = original.update(None, None, None, None, None, None);

        assert_eq!(updated.name, original.name);
        assert_eq!(updated.sex, original.sex);
        assert_eq!(updated.races, original.races);
        assert_eq!(updated.birthdate, original.birthdate);
        assert_eq!(updated.weight, original.weight);
        assert_eq!(updated.icad_id, original.icad_id);
    }

    #[test]
    fn test_update_dog_replace_birthdate() {
        let original = Dog::new(
            "Bella".to_string(),
            Sex::F,
            None,
            vec!["Border Collie".to_string()],
            None,
            None,
        );

        let new_birthdate = Some(Utc::now());

        let updated = original.update(None, None, Some(new_birthdate), None, None, None);

        assert_eq!(updated.birthdate, new_birthdate);
    }

    #[test]
    fn test_update_dog_keep_existing_birthdate() {
        let existing_birthdate = Some(Utc::now());

        let original = Dog::new(
            "Fido".to_string(),
            Sex::M,
            existing_birthdate,
            vec!["Berger".to_string()],
            Some(20),
            None,
        );

        let updated = original.update(
            Some("Fidou".to_string()),
            None,
            None, // birthdate non changée
            None,
            None,
            None,
        );

        assert_eq!(updated.birthdate, existing_birthdate); // inchangé
        assert_eq!(updated.name, "Fidou");
    }
}
