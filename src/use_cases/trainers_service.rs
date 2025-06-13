use chrono::{DateTime, Utc};
use thiserror::Error;
use uuid::Uuid;

use crate::{
    entities::trainers::{Trainer, TrainerConnection},
    repositories::trainers_repository::{ListTrainerInput, TrainersRepository},
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

#[derive(Debug, Clone)]
pub struct UpdateTrainerInput {
    pub id: Uuid,
    pub name: Option<String>,
    pub birthdate: Option<Option<DateTime<Utc>>>,
    pub sex: Option<Sex>,
    pub contact_email: Option<Option<String>>,
    pub phone_number: Option<Option<String>>,
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

    pub async fn update_trainer(
        &self,
        input: UpdateTrainerInput,
    ) -> Result<Trainer, TrainerServiceError> {
        let trainer = self
            .repo
            .get_trainer(input.id)
            .await?
            .ok_or(TrainerServiceError::NotFound)?;

        let updated_trainer = trainer.update(
            input.name,
            input.sex,
            input.birthdate,
            input.contact_email,
            input.phone_number,
        );

        let repo_result = self
            .repo
            .update_trainer(updated_trainer.clone())
            .await
            .map_err(TrainerServiceError::from);

        match repo_result {
            Ok(_) => Ok(updated_trainer),
            Err(e) => Err(e),
        }
    }

    pub async fn list_trainers(
        &self,
        input: ListTrainerInput,
    ) -> Result<TrainerConnection, TrainerServiceError> {
        self.repo
            .list_trainers(input)
            .await
            .map_err(TrainerServiceError::from)
    }
}
