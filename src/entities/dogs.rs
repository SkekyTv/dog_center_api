use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::shared::types::sex::Sex;

use super::shared::page_info::PageInfo;

pub struct DogConnection {
    pub edges: Vec<DogEdge>,
    pub page_info: PageInfo,
}
pub struct DogEdge {
    pub cursor: String,
    pub node: Dog,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Dog {
    pub id: Uuid,
    pub name: String,
    pub birthdate: Option<DateTime<Utc>>,
    pub races: Vec<String>,
    pub weight: Vec<i32>,
    pub img_url: Option<String>, // unimplemented!()
    pub sex: Sex,
    pub icad_id: Option<String>,
    pub desactivated_at: Option<DateTime<Utc>>,
}

impl Dog {
    pub fn new(
        name: String,
        sex: Sex,
        birthdate: Option<DateTime<Utc>>,
        races: Vec<String>,
        weight: Vec<i32>,
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
            desactivated_at: None,
        }
    }

    pub fn update(
        &self,
        name: Option<String>,
        sex: Option<Sex>,
        birthdate: Option<Option<DateTime<Utc>>>,
        races: Option<Vec<String>>,
        weight: Option<Vec<i32>>,
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
            weight: weight.unwrap_or_else(|| self.weight.clone()),
            img_url: self.img_url.clone(),
            icad_id: match icad_id {
                Some(inner) => inner,
                None => self.icad_id.clone(),
            },
            desactivated_at: self.desactivated_at,
        }
    }

    pub fn toggle_activation_status(&self) -> Self {
        let mut dog = self.clone();

        dog.desactivated_at = match self.desactivated_at {
            Some(_) => None,
            None => Some(Utc::now()),
        };

        dog
    }
}

#[cfg(test)]
mod tests {
    use std::{thread::sleep, time::Duration};

    use super::*;

    #[test]
    fn test_dog_constructor_minimal_params() {
        let dog = Dog::new(
            "pupuce".to_string(),
            Sex::F,
            None,
            [].to_vec(),
            vec![100],
            None,
        );
        assert_eq!(dog.name, "pupuce");
        assert_eq!(dog.sex, Sex::F);
        assert_eq!(dog.weight[0], 100);
    }

    #[test]
    fn test_update_dog_remove_weight() {
        let original = Dog::new(
            "Buddy".to_string(),
            Sex::M,
            None,
            vec!["Golden Retriever".to_string()],
            vec![30],
            None,
        );

        let updated = original.update(None, None, None, None, Some(vec![]), None);

        assert_eq!(updated.weight, Vec::<i32>::new());
    }

    #[test]
    fn test_update_dog_partial_update() {
        let original = Dog::new(
            "Rex".to_string(),
            Sex::M,
            Some(Utc::now()),
            vec!["Labrador".to_string()],
            vec![25],
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
            vec![10],
            Some("ICAD456".to_string()),
        );

        let updated = original.update(
            None,
            None,
            None,
            None,
            None,
            Some(None), // on retire l'ICAD
        );

        assert_eq!(updated.name, original.name);
        assert_eq!(updated.icad_id, None);
    }

    #[test]
    fn test_update_dog_no_changes() {
        let original = Dog::new(
            "Rex".to_string(),
            Sex::M,
            None,
            vec!["Labrador".to_string()],
            vec![30],
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
            vec![],
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
            vec![20],
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

    #[test]
    fn toggles_from_none_to_some() {
        let dog = Dog::new(
            "Bella".to_string(),
            Sex::F,
            None,
            vec!["Border Collie".to_string()],
            vec![],
            None,
        );
        let toggled = dog.toggle_activation_status();

        assert!(dog.desactivated_at.is_none());
        assert!(toggled.desactivated_at.is_some());

        // Vérifie que la date est proche de maintenant (tolérance faible)
        let now = Utc::now();
        assert!(toggled.desactivated_at.unwrap() <= now);
    }

    #[test]
    fn toggles_from_some_to_none() {
        let dog = Dog::new(
            "Bella".to_string(),
            Sex::F,
            None,
            vec!["Border Collie".to_string()],
            vec![],
            None,
        );

        let toggled_to_some = dog.toggle_activation_status();
        sleep(Duration::from_millis(10));

        let toggled_to_none = toggled_to_some.toggle_activation_status();

        assert!(toggled_to_some.desactivated_at.is_some());
        assert!(toggled_to_none.desactivated_at.is_none());
    }

    #[test]
    fn toggle_is_idempotent_over_two_calls() {
        let dog = Dog::new(
            "Bella".to_string(),
            Sex::F,
            None,
            vec!["Border Collie".to_string()],
            vec![],
            None,
        );
        let toggled = dog.toggle_activation_status();
        let toggled_back = toggled.toggle_activation_status();

        assert_eq!(dog.desactivated_at, toggled_back.desactivated_at);
    }
}
