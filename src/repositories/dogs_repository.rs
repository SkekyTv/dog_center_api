use crate::entities::dogs::{Dog, DogConnection};
use async_trait::async_trait;
use uuid::Uuid;

#[derive(Debug)]
pub struct ListDogInput {
    pub after_id: Option<String>,
    pub first: i32,
}

#[async_trait]
pub trait DogsRepository: Send + Sync {
    async fn get_dog(&self, id: Uuid) -> Result<Option<Dog>, sqlx::Error>;

    async fn create_dog(&self, dog: Dog) -> Result<(), sqlx::Error>;

    async fn update_dog(&self, dog: Dog) -> Result<(), sqlx::Error>;

    async fn toggle_activation_status(&self, dog: Dog) -> Result<(), sqlx::Error>;

    async fn list_dogs(&self, input: ListDogInput) -> Result<DogConnection, sqlx::Error>;
}
