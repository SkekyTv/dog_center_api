use crate::entities::dogs::Dogs;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait DogsRepository: Send + Sync {
    async fn get_dog(&self, id: Uuid) -> Result<Option<Dogs>, sqlx::Error>;

    async fn create_dog(&self, dog: Dogs) -> Result<(), sqlx::Error>;
}
