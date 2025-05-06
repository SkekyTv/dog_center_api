use crate::entities::dogs::Dog;
use crate::repositories::dogs_repository::DogsRepository;
use uuid::Uuid;

pub struct DogsService<T: DogsRepository> {
    pub repo: T,
}

impl<T: DogsRepository> DogsService<T> {
    pub fn new(repo: T) -> Self {
        Self { repo }
    }

    pub async fn get_dog(&self, id: Uuid) -> Result<Option<Dog>, sqlx::Error> {
        self.repo.get_dog(id).await
    }

    pub async fn create_dog(&self, dog: Dog) -> Result<(), sqlx::Error> {
        self.repo.create_dog(dog).await
    }
}
