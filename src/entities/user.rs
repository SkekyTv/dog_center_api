use argon2::{
    Argon2,
    password_hash::{
        Error as PasswordHashError, PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
        rand_core::OsRng,
    },
};
use garde::Validate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Validate, Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    #[garde(skip)]
    pub id: Uuid,

    #[garde(email)]
    pub email: String,

    #[garde(skip)]
    pub pdw_hash: String,
}

impl User {
    pub fn new(email: String, pdw: String) -> Result<Self, garde::Error> {
        let pdw_hash = User::hash_password(&pdw).map_err(|e| garde::Error::new(e.to_string()))?;
        let user = User {
            id: Uuid::new_v4(),
            email,
            pdw_hash,
        };

        match user.validate() {
            Ok(_) => Ok(user),
            Err(report) => Err(garde::Error::new(report.to_string())), // Adjust conversion logic
        }
    }
    fn hash_password(password: &str) -> Result<String, PasswordHashError> {
        let salt = SaltString::generate(&mut OsRng);
        // Argon2 with default params (Argon2id v19)
        let argon2 = Argon2::default();
        // Hash password to PHC string ($argon2id$v=19$...)
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)?
            .to_string();

        Ok(password_hash)
    }

    pub fn verify_password(&self, password: &str, hash: &str) -> Result<bool, PasswordHashError> {
        let parsed_hash = PasswordHash::new(hash)?;
        let argon2 = Argon2::default();
        match argon2.verify_password(password.as_bytes(), &parsed_hash) {
            Ok(_) => Ok(true),
            Err(PasswordHashError::Password) => Ok(false),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_new_valid_email() {
        let email = "test@example.com".to_string();
        let password = "StrongPassword123!".to_string();
        let user = User::new(email.clone(), password.clone());
        assert!(user.is_ok());
        let user = user.unwrap();
        assert_eq!(user.email, email);
        assert!(!user.pdw_hash.is_empty());
    }

    #[test]
    fn test_user_new_invalid_email() {
        let email = "invalid-email".to_string();
        let password = "StrongPassword123!".to_string();
        let user = User::new(email, password);
        assert!(user.is_err());
    }

    #[test]
    fn test_hash_password_and_verify() {
        let password = "AnotherStrongPassword!".to_string();
        let hash = User::hash_password(&password).unwrap();
        let user = User {
            id: Uuid::new_v4(),
            email: "foo@bar.com".to_string(),
            pdw_hash: hash.clone(),
        };
        let is_valid = user.verify_password(&password, &hash).unwrap();
        assert!(is_valid);

        let is_invalid = user.verify_password("wrongpassword", &hash).unwrap();
        assert!(!is_invalid);
    }

    #[test]
    fn test_verify_password_with_invalid_hash() {
        let user = User {
            id: Uuid::new_v4(),
            email: "foo@bar.com".to_string(),
            pdw_hash: "not_a_real_hash".to_string(),
        };
        let result = user.verify_password("password", "not_a_real_hash");
        assert!(result.is_err());
    }
}
