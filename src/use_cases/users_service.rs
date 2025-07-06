use std::sync::Arc;

use garde::Validate;
use thiserror::Error;

use crate::entities::user::{Authorize, User};
use crate::repositories::users_repository::UsersRepository;
use crate::shared::validators::is_strong_password::{PasswordContext, is_strong_password};

use super::jwt_service::JwtServiceTrait;

#[derive(Debug, Error)]
pub enum UserServiceError {
    #[error("UserNotFound")]
    NotFound,
    #[error("Database error: {0}")]
    DbError(#[from] sqlx::Error),
    #[error("User validation failed: {0}")]
    ValidationError(#[from] garde::Error),
    #[error("InvalidPassword")]
    InvalidPdw,
    #[error("Token serialization failed")]
    TokenSerializationFailed,
}

#[derive(Debug, Clone, Validate)]
#[garde(context(PasswordContext))]
pub struct SignUpInput {
    #[garde(email)]
    pub email: String,
    #[garde(custom(is_strong_password))]
    pub pdw: String,
}

#[derive(Debug, Clone, Validate)]
pub struct LoginInput {
    #[garde(email)]
    pub email: String,
    #[garde(skip)]
    pub pdw: String,
}
pub struct UsersService<T: UsersRepository, J: JwtServiceTrait + ?Sized> {
    pub repo: T,
    pub jwt_service: Arc<J>,
}

impl<T: UsersRepository, J: JwtServiceTrait + ?Sized> UsersService<T, J> {
    pub fn new(repo: T, jwt_service: Arc<J>) -> Self {
        Self { repo, jwt_service }
    }

    pub async fn sign_up(&self, input: SignUpInput) -> Result<User, UserServiceError> {
        let user = User::new(input.email, input.pdw)?;

        self.repo
            .create_user(user.clone())
            .await
            .map_err(UserServiceError::from)?;

        Ok(user)
    }

    pub async fn login(&self, input: LoginInput) -> Result<Authorize, UserServiceError> {
        let user = self
            .repo
            .get_user_by_email(input.email)
            .await
            .map_err(UserServiceError::from)?;

        match user {
            Some(u) => {
                let is_good_pdw = u.verify_password(&input.pdw);
                match is_good_pdw {
                    Ok(b) => {
                        if b {
                            let token = self
                                .jwt_service
                                .encode(u.id.to_string())
                                .map_err(|_| UserServiceError::TokenSerializationFailed)?;
                            Ok(Authorize { token })
                        } else {
                            Err(UserServiceError::InvalidPdw)
                        }
                    }
                    Err(_) => Err(UserServiceError::InvalidPdw),
                }
            }
            None => Err(UserServiceError::NotFound),
        }
    }
}
