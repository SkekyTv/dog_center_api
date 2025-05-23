use async_trait::async_trait;
use sqlx::PgPool;
use tracing::{error, info};
use uuid::Uuid;

use crate::entities::dogs::Dog;
use crate::repositories::dogs_repository::DogsRepository;

#[derive(Clone)]
pub struct PgDogsRepository {
    pub pool: PgPool,
}

#[async_trait]
impl DogsRepository for PgDogsRepository {
    async fn get_dog(&self, id: Uuid) -> Result<Option<Dog>, sqlx::Error> {
        sqlx::query_as::<_, Dog>(
            "SELECT id::Uuid, name::Text, birthdate, races, img_url, sex, weight, icad_id, desactivated_at FROM dogs WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
    }

    async fn create_dog(&self, dog: Dog) -> Result<(), sqlx::Error> {
        let result =sqlx::query(
            "INSERT INTO dogs (id, name, birthdate, races, img_url, sex, weight, icad_id) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
        )
            .bind(dog.id)
            .bind(&dog.name)
            .bind(dog.birthdate)
            .bind(&dog.races)
            .bind(dog.img_url)
            .bind(&dog.sex)
            .bind(dog.weight)
            .bind(dog.icad_id)
        .execute(&self.pool)
        .await;
        match result {
            Ok(_) => info!("Success"),
            Err(e) => error!("error create_dog : {}", e),
        }

        Ok(())
    }

    async fn update_dog(&self, dog: Dog) -> Result<(), sqlx::Error> {
        let result = sqlx::query(
            "UPDATE dogs SET name = $1, birthdate = $2, races = $3, img_url = $4, sex = $5, weight = $6, icad_id= $7 WHERE id = $8"
            )
            .bind(dog.name)
            .bind(dog.birthdate)
            .bind(dog.races)
            .bind(dog.img_url)
            .bind(dog.sex)
            .bind(dog.weight)
            .bind(dog.icad_id)
            .bind(dog.id)
            .execute(&self.pool)
            .await;
        match result {
            Ok(_) => info!("Success"),
            Err(e) => error!("error update_dog : {}", e),
        }

        Ok(())
    }

    async fn toggle_activation_status(&self, dog: Dog) -> Result<(), sqlx::Error> {
        let result = sqlx::query("UPDATE dogs SET desactivated_at = $1 WHERE id = $2")
            .bind(dog.desactivated_at)
            .bind(dog.id)
            .execute(&self.pool)
            .await;

        match result {
            Ok(_) => info!("Success"),
            Err(e) => error!("error toogle_activation_status : {}", e),
        }

        Ok(())
    }
}
