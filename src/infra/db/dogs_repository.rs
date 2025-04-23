use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::entities::dogs::Dogs;
use crate::repositories::dogs_repository::DogsRepository;

#[derive(Clone)]
pub struct PgDogsRepository {
    pub pool: PgPool,
}

#[async_trait]
impl DogsRepository for PgDogsRepository {
    async fn get_dog(&self, id: Uuid) -> Result<Option<Dogs>, sqlx::Error> {
        sqlx::query_as!(
            Dogs,
            "SELECT id::Uuid, name::Text, birthdate, races, img_url FROM dogs WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await
    }

    async fn create_dog(&self, dog: Dogs) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO dogs VALUES ($1, $2, $3, $4, $5)",
            dog.id,
            dog.name,
            dog.birthdate,
            &dog.races,
            dog.img_url
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
