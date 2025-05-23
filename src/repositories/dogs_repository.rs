use crate::entities::dogs::Dog;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait DogsRepository: Send + Sync {
    async fn get_dog(&self, id: Uuid) -> Result<Option<Dog>, sqlx::Error>;

    async fn create_dog(&self, dog: Dog) -> Result<(), sqlx::Error>;

    async fn update_dog(&self, dog: Dog) -> Result<(), sqlx::Error>;

    async fn toggle_activation_status(&self, dog: Dog) -> Result<(), sqlx::Error>;
}
