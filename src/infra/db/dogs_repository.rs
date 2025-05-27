use async_trait::async_trait;
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use sqlx::PgPool;
use tracing::{error, info};
use uuid::Uuid;

use crate::entities::dogs::{Dog, DogConnection, DogEdge};
use crate::entities::shared::page_info::PageInfo;
use crate::repositories::dogs_repository::{DogsRepository, ListDogInput};

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

    async fn list_dogs(&self, input: ListDogInput) -> Result<DogConnection, sqlx::Error> {
        println!("input db : {:?}", input);
        let result = sqlx::query_as::<_, Dog>("SELECT id::Uuid, name::Text, birthdate, races, img_url, sex, weight, icad_id FROM dogs WHERE ($1 IS NULL OR id > $1::uuid) ORDER BY id ASC LIMIT $2")
            .bind(input.after_id)
            .bind(input.first + 1) // to check next page
            .fetch_all(&self.pool)
        .await;

        let dogs = match result {
            Ok(dogs) => dogs,
            Err(e) => {
                error!("error list_dogs: {}", e);
                return Err(e);
            }
        };

        let has_next_page = dogs.len() > input.first as usize;

        let items = dogs
            .into_iter()
            .take(input.first as usize)
            .collect::<Vec<_>>();

        let edges: Vec<DogEdge> = items
            .iter()
            .map(|dog| DogEdge {
                cursor: BASE64_STANDARD.encode(dog.id.to_string()),
                node: dog.clone(),
            })
            .collect();

        let end_cursor = edges.last().map(|e| e.cursor.clone());

        Ok(DogConnection {
            edges,
            page_info: PageInfo {
                end_cursor,
                has_next_page,
            },
        })
    }
}
