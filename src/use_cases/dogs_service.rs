use crate::entities::dogs::DogConnection;
use crate::repositories::dogs_repository::DogsRepository;
use crate::shared::types::sex::Sex;
use crate::{entities::dogs::Dog, repositories::dogs_repository::ListDogInput};
use chrono::{DateTime, Utc};
use thiserror::Error;
use uuid::Uuid;

pub struct DogsService<T: DogsRepository> {
    pub repo: T,
}

pub struct UpdateDogInput {
    pub id: uuid::Uuid,
    pub name: Option<String>,
    pub birthdate: Option<Option<DateTime<Utc>>>,
    pub races: Option<Vec<String>>,
    pub weight: Option<Vec<i32>>,
    pub sex: Option<Sex>,
    pub icad_id: Option<Option<String>>,
}

#[derive(Debug, Clone)]
pub struct CreateDogInput {
    pub name: String,
    pub birthdate: Option<DateTime<Utc>>,
    pub races: Vec<String>,
    pub weight: Vec<i32>,
    pub sex: Sex,
    pub icad_id: Option<String>,
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

    pub async fn create_dog(&self, input: CreateDogInput) -> Result<Dog, DogServiceError> {
        let dog = Dog::new(
            input.name,
            input.sex,
            input.birthdate,
            input.races,
            input.weight,
            input.icad_id,
        );

        let repo_result = self
            .repo
            .create_dog(dog.clone())
            .await
            .map_err(DogServiceError::from);

        match repo_result {
            Ok(_) => Ok(dog),
            Err(e) => Err(e),
        }
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

    pub async fn toggle_activation_status(&self, id: Uuid) -> Result<Dog, DogServiceError> {
        let dog = self
            .repo
            .get_dog(id)
            .await?
            .ok_or(DogServiceError::NotFound)?;

        println!("Dog read: {:?}", dog);
        let toggled_dog = dog.toggle_activation_status();
        println!("toggled dog: {:?}", toggled_dog);

        let repo_result = self
            .repo
            .toggle_activation_status(toggled_dog.clone())
            .await
            .map_err(DogServiceError::from);

        match repo_result {
            Ok(_) => Ok(toggled_dog),
            Err(e) => Err(e),
        }
    }

    pub async fn list_dogs(&self, input: ListDogInput) -> Result<DogConnection, DogServiceError> {
        self.repo
            .list_dogs(input)
            .await
            .map_err(DogServiceError::from)
    }
}
