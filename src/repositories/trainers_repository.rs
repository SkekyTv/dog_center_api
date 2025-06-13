use async_trait::async_trait;
use uuid::Uuid;

use crate::entities::trainers::{Trainer, TrainerConnection};

#[derive(Debug)]
pub struct ListTrainerInput {
    pub after_id: Option<String>,
    pub first: i32,
}

#[async_trait]
pub trait TrainersRepository: Send + Sync {
    async fn get_trainer(&self, id: Uuid) -> Result<Option<Trainer>, sqlx::Error>;

    async fn create_trainer(&self, trainer: Trainer) -> Result<(), sqlx::Error>;

    async fn update_trainer(&self, trainer: Trainer) -> Result<(), sqlx::Error>;

    async fn list_trainers(
        &self,
        input: ListTrainerInput,
    ) -> Result<TrainerConnection, sqlx::Error>;
}
