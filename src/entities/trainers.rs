use chrono::{DateTime, Utc};
use garde::Validate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::shared::types::sex::Sex;

#[derive(Validate, Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
#[garde(allow_unvalidated)]
pub struct Trainer {
    pub id: Uuid,

    #[garde(email)]
    pub contact_email: Option<String>,

    #[garde(length(min = 1, max = 100))]
    pub name: String,

    #[garde(phone_number)]
    pub phone_number: Option<String>,

    pub birthdate: Option<DateTime<Utc>>,

    pub sex: Sex,

    #[garde(url)]
    pub img_url: Option<String>, //unimplemented!()
}

impl Trainer {
    pub fn new(
        name: String,
        sex: Sex,
        birthdate: Option<DateTime<Utc>>,
        contact_email: Option<String>,
        phone_number: Option<String>,
    ) -> Result<Self, garde::Error> {
        let trainer = Trainer {
            id: Uuid::new_v4(),
            name,
            birthdate,
            contact_email,
            phone_number,
            sex,
            img_url: None,
        };

        match trainer.validate() {
            Ok(_) => Ok(trainer),
            Err(report) => Err(garde::Error::new(report.to_string())), // Adjust conversion logic
        }
    }

    pub fn update(
        &self,
        name: Option<String>,
        sex: Option<Sex>,
        birthdate: Option<Option<DateTime<Utc>>>,
        contact_email: Option<Option<String>>,
        phone_number: Option<Option<String>>,
    ) -> Self {
        Trainer {
            id: self.id,
            name: name.unwrap_or_else(|| self.name.clone()),
            sex: sex.unwrap_or(self.sex),
            birthdate: match birthdate {
                Some(inner) => inner,
                None => self.birthdate,
            },
            contact_email: contact_email.unwrap_or_else(|| self.contact_email.clone()),
            phone_number: phone_number.unwrap_or_else(|| self.phone_number.clone()),
            img_url: None,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::shared::types::sex::Sex;
    use chrono::Utc;

    #[test]
    fn test_valid_trainer_creation() {
        let name = "John Doe".to_string();
        let sex = Sex::M;
        let birthdate = Some(Utc::now());
        let contact_email = Some("john.doe@example.com".to_string());
        let phone_number = Some("+33612345678".to_string());

        let result = Trainer::new(name, sex, birthdate, contact_email, phone_number);
        println!("error: {:?}", result);

        assert!(result.is_ok());
        let trainer = result.unwrap();
        assert_eq!(trainer.name, "John Doe");
        assert_eq!(trainer.contact_email.unwrap(), "john.doe@example.com");
        assert_eq!(trainer.phone_number.unwrap(), "+33612345678");
    }

    #[test]
    fn test_invalid_email() {
        let name = "John Doe".to_string();
        let sex = Sex::M;
        let birthdate = Some(Utc::now());
        let contact_email = Some("invalid-email".to_string());
        let phone_number = Some("+1234567890".to_string());

        let result = Trainer::new(name, sex, birthdate, contact_email, phone_number);

        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_name_length() {
        let name = "".to_string(); // Invalid name (too short)
        let sex = Sex::M;
        let birthdate = Some(Utc::now());
        let contact_email = Some("john.doe@example.com".to_string());
        let phone_number = Some("+1234567890".to_string());

        let result = Trainer::new(name, sex, birthdate, contact_email, phone_number);

        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_phone_number() {
        let name = "John Doe".to_string();
        let sex = Sex::M;
        let birthdate = Some(Utc::now());
        let contact_email = Some("john.doe@example.com".to_string());
        let phone_number = Some("invalid-phone".to_string()); // Invalid phone number

        let result = Trainer::new(name, sex, birthdate, contact_email, phone_number);

        assert!(result.is_err());
    }

    #[test]
    fn test_missing_optional_fields() {
        let name = "John Doe".to_string();
        let sex = Sex::M;
        let birthdate = None; // Missing birthdate
        let contact_email = None; // Missing email
        let phone_number = None; // Missing phone number

        let result = Trainer::new(name, sex, birthdate, contact_email, phone_number);

        assert!(result.is_ok());
        let trainer = result.unwrap();
        assert_eq!(trainer.name, "John Doe");
        assert!(trainer.contact_email.is_none());
        assert!(trainer.phone_number.is_none());
    }

    #[test]
    fn test_update_all_fields() {
        // Arrange
        let original_trainer = Trainer::new(
            "John Doe".to_string(),
            Sex::M,
            Some(Utc::now()),
            Some("john@example.com".to_string()),
            Some("+33612345678".to_string()),
        )
        .unwrap();

        let new_birthdate = Some(Utc::now() - chrono::Duration::days(365));

        // Act
        let updated_trainer = original_trainer.update(
            Some("Jane Smith".to_string()),
            Some(Sex::F),
            Some(new_birthdate),
            Some(Some("jane@example.com".to_string())),
            Some(Some("+33687654321".to_string())),
        );

        // Assert
        assert_eq!(updated_trainer.id, original_trainer.id);
        assert_eq!(updated_trainer.name, "Jane Smith");
        assert_eq!(updated_trainer.sex, Sex::F);
        assert_eq!(updated_trainer.birthdate, new_birthdate);
        assert_eq!(
            updated_trainer.contact_email,
            Some("jane@example.com".to_string())
        );
        assert_eq!(
            updated_trainer.phone_number,
            Some("+33687654321".to_string())
        );
        assert_eq!(updated_trainer.img_url, None);
    }

    #[test]
    fn test_update_partial_fields() {
        // Arrange
        let original_trainer = Trainer::new(
            "John Doe".to_string(),
            Sex::M,
            Some(Utc::now()),
            Some("john@example.com".to_string()),
            Some("+33612345678".to_string()),
        )
        .unwrap();

        // Act - update only name and sex
        let updated_trainer =
            original_trainer.update(Some("Jane Doe".to_string()), Some(Sex::F), None, None, None);

        // Assert
        assert_eq!(updated_trainer.id, original_trainer.id);
        assert_eq!(updated_trainer.name, "Jane Doe");
        assert_eq!(updated_trainer.sex, Sex::F);
        assert_eq!(updated_trainer.birthdate, original_trainer.birthdate);
        assert_eq!(
            updated_trainer.contact_email,
            original_trainer.contact_email
        );
        assert_eq!(updated_trainer.phone_number, original_trainer.phone_number);
    }

    #[test]
    fn test_update_no_changes() {
        // Arrange
        let original_trainer = Trainer::new(
            "John Doe".to_string(),
            Sex::M,
            Some(Utc::now()),
            Some("john@example.com".to_string()),
            Some("+33612345678".to_string()),
        )
        .unwrap();

        // Act - no updates
        let updated_trainer = original_trainer.update(None, None, None, None, None);

        // Assert
        assert_eq!(updated_trainer.id, original_trainer.id);
        assert_eq!(updated_trainer.name, original_trainer.name);
        assert_eq!(updated_trainer.sex, original_trainer.sex);
        assert_eq!(updated_trainer.birthdate, original_trainer.birthdate);
        assert_eq!(
            updated_trainer.contact_email,
            original_trainer.contact_email
        );
        assert_eq!(updated_trainer.phone_number, original_trainer.phone_number);
    }

    #[test]
    fn test_update_optional_fields_to_none() {
        // Arrange
        let original_trainer = Trainer::new(
            "John Doe".to_string(),
            Sex::M,
            Some(Utc::now()),
            Some("john@example.com".to_string()),
            Some("+33612345678".to_string()),
        )
        .unwrap();

        // Act - set optional fields to None
        let updated_trainer = original_trainer.update(
            None,
            None,
            Some(None), // Set birthdate to None
            Some(None), // Set contact_email to None
            Some(None), // Set phone_number to None
        );

        // Assert
        assert_eq!(updated_trainer.id, original_trainer.id);
        assert_eq!(updated_trainer.name, original_trainer.name);
        assert_eq!(updated_trainer.sex, original_trainer.sex);
        assert_eq!(updated_trainer.birthdate, None);
        assert_eq!(updated_trainer.contact_email, None);
        assert_eq!(updated_trainer.phone_number, None);
    }

    #[test]
    fn test_update_from_none_to_some() {
        // Arrange
        let original_trainer =
            Trainer::new("John Doe".to_string(), Sex::M, None, None, None).unwrap();

        let new_birthdate = Utc::now();

        // Act - add values to previously None fields
        let updated_trainer = original_trainer.update(
            None,
            None,
            Some(Some(new_birthdate)),
            Some(Some("new@example.com".to_string())),
            Some(Some("+33600000000".to_string())),
        );

        // Assert
        assert_eq!(updated_trainer.birthdate, Some(new_birthdate));
        assert_eq!(
            updated_trainer.contact_email,
            Some("new@example.com".to_string())
        );
        assert_eq!(
            updated_trainer.phone_number,
            Some("+33600000000".to_string())
        );
    }

    #[test]
    fn test_update_preserves_id() {
        // Arrange
        let original_trainer =
            Trainer::new("John Doe".to_string(), Sex::M, None, None, None).unwrap();

        let original_id = original_trainer.id;

        // Act - multiple updates
        let updated_trainer1 =
            original_trainer.update(Some("New Name".to_string()), None, None, None, None);
        let updated_trainer2 = updated_trainer1.update(None, Some(Sex::F), None, None, None);

        // Assert
        assert_eq!(updated_trainer1.id, original_id);
        assert_eq!(updated_trainer2.id, original_id);
    }

    #[test]
    fn test_update_always_resets_img_url() {
        // Arrange - create a trainer and manually set img_url
        let mut original_trainer =
            Trainer::new("John Doe".to_string(), Sex::M, None, None, None).unwrap();
        original_trainer.img_url = Some("http://example.com/image.jpg".to_string());

        // Act
        let updated_trainer = original_trainer.update(None, None, None, None, None);

        // Assert - img_url should be reset to None
        assert_eq!(updated_trainer.img_url, None);
    }
}
