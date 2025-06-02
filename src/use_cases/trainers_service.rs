use chrono::{DateTime, Utc};
use thiserror::Error;
use uuid::Uuid;

use crate::{
    entities::trainers::Trainer, repositories::trainers_repository::TrainersRepository,
    shared::types::sex::Sex,
};

pub struct TrainersService<T: TrainersRepository> {
    pub repo: T,
}

#[derive(Debug, Error)]
pub enum TrainerServiceError {
    #[error("TrainerNotFound")]
    NotFound,
    #[error("Database error: {0}")]
    DbError(#[from] sqlx::Error),
    #[error("Trainer validation failed: {0}")]
    ValidationError(#[from] garde::Error),
}

#[derive(Debug, Clone)]
pub struct CreateTrainerInput {
    pub name: String,
    pub birthdate: Option<DateTime<Utc>>,
    pub sex: Sex,
    pub contact_email: Option<String>,
    pub phone_number: Option<String>,
}

impl<T: TrainersRepository> TrainersService<T> {
    pub fn new(repo: T) -> Self {
        Self { repo }
    }

    pub async fn get_trainer(&self, id: Uuid) -> Result<Option<Trainer>, TrainerServiceError> {
        self.repo
            .get_trainer(id)
            .await
            .map_err(TrainerServiceError::from)
    }

    pub async fn create_trainer(
        &self,
        input: CreateTrainerInput,
    ) -> Result<Trainer, TrainerServiceError> {
        let trainer = Trainer::new(
            input.name,
            input.sex,
            input.birthdate,
            input.contact_email,
            input.phone_number,
        )?;

        self.repo
            .create_trainer(trainer.clone())
            .await
            .map_err(TrainerServiceError::from)?;

        Ok(trainer)
    }
}
