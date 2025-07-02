use garde::Validate;
use thiserror::Error;

use crate::entities::user::User;
use crate::repositories::users_repository::UsersRepository;
use crate::shared::validators::is_strong_password::{PasswordContext, is_strong_password};

#[derive(Debug, Error)]
pub enum UserServiceError {
    #[error("UserNotFound")]
    NotFound,
    #[error("Database error: {0}")]
    DbError(#[from] sqlx::Error),
    #[error("User validation failed: {0}")]
    ValidationError(#[from] garde::Error),
}

#[derive(Debug, Clone, Validate)]
#[garde(context(PasswordContext))]
pub struct SignUpInput {
    #[garde(email)]
    pub email: String,
    #[garde(custom(is_strong_password))]
    pub pdw: String,
}
pub struct UsersService<T: UsersRepository> {
    pub repo: T,
}

impl<T: UsersRepository> UsersService<T> {
    pub fn new(repo: T) -> Self {
        Self { repo }
    }

    pub async fn sign_up(&self, input: SignUpInput) -> Result<User, UserServiceError> {
        let user = User::new(input.email, input.pdw)?;

        self.repo
            .create_user(user.clone())
            .await
            .map_err(UserServiceError::from)?;

        Ok(user)
    }
}
