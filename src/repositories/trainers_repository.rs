use async_trait::async_trait;
use uuid::Uuid;

use crate::entities::trainers::Trainer;

#[async_trait]
pub trait TrainersRepository: Send + Sync {
    async fn get_trainer(&self, id: Uuid) -> Result<Option<Trainer>, sqlx::Error>;

    async fn create_trainer(&self, trainer: Trainer) -> Result<(), sqlx::Error>;

    async fn update_trainer(&self, trainer: Trainer) -> Result<(), sqlx::Error>;
}
