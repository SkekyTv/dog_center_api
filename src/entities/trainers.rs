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
}
