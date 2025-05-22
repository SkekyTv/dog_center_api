use crate::entities::dogs::Dog;
use crate::repositories::dogs_repository::DogsRepository;
use crate::shared::types::sex::Sex;
use chrono::{DateTime, Utc};
use thiserror::Error;
use uuid::Uuid; // pour #[derive(Error)]

pub struct DogsService<T: DogsRepository> {
    pub repo: T,
}

pub struct UpdateDogInput {
    pub id: uuid::Uuid,
    pub name: Option<String>,
    pub birthdate: Option<Option<DateTime<Utc>>>,
    pub races: Option<Vec<String>>,
    pub weight: Option<Option<i32>>,
    pub sex: Option<Sex>,
    pub icad_id: Option<Option<String>>,
}

#[derive(Debug, Error)]
pub enum DogServiceError {
    #[error("Dog not found")]
    NotFound,
    #[error("Database error: {0}")]
    DbError(#[from] sqlx::Error),
}

impl<T: DogsRepository> DogsService<T> {
    pub fn new(repo: T) -> Self {
        Self { repo }
    }

    pub async fn get_dog(&self, id: Uuid) -> Result<Option<Dog>, DogServiceError> {
        self.repo.get_dog(id).await.map_err(DogServiceError::from)
    }

    pub async fn create_dog(&self, dog: Dog) -> Result<(), DogServiceError> {
        self.repo
            .create_dog(dog)
            .await
            .map_err(DogServiceError::from)
    }

    pub async fn update_dog(&self, input: UpdateDogInput) -> Result<Dog, DogServiceError> {
        let dog = self
            .repo
            .get_dog(input.id)
            .await?
            .ok_or(DogServiceError::NotFound)?;

        let updated_dog = dog.update(
            input.name,
            input.sex,
            input.birthdate,
            input.races,
            input.weight,
            input.icad_id,
        );

        let repo_result = self
            .repo
            .update_dog(updated_dog.clone())
            .await
            .map_err(DogServiceError::from);

        match repo_result {
            Ok(_) => Ok(updated_dog),
            Err(e) => Err(e),
        }
    }
}
