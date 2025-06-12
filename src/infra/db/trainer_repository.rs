use async_trait::async_trait;
use sqlx::PgPool;
use tracing::{error, info};
use uuid::Uuid;

use crate::{entities::trainers::Trainer, repositories::trainers_repository::TrainersRepository};

#[derive(Clone)]
pub struct PgTrainersRepository {
    pub pool: PgPool,
}

#[async_trait]
impl TrainersRepository for PgTrainersRepository {
    async fn get_trainer(&self, id: Uuid) -> Result<Option<Trainer>, sqlx::Error> {
        sqlx::query_as::<_, Trainer>(
            "SELECT id::Uuid, name::Text, birthdate,  img_url, sex, contact_email, phone_number FROM trainers WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
    }

    async fn create_trainer(&self, trainer: Trainer) -> Result<(), sqlx::Error> {
        let result =sqlx::query(
            "INSERT INTO trainers (id, name, birthdate,  img_url, sex, contact_email, phone_number) VALUES ($1, $2, $3, $4, $5, $6, $7)",
        )
            .bind(trainer.id)
            .bind(&trainer.name)
            .bind(trainer.birthdate)
            .bind(trainer.img_url)
            .bind(trainer.sex)
            .bind(trainer.contact_email)
            .bind(trainer.phone_number)
        .execute(&self.pool)
        .await;
        match result {
            Ok(_) => {
                info!("Success");
                Ok(())
            }
            Err(e) => {
                error!("error create_trainer : {}", e);
                Err(e)
            }
        }
    }

    async fn update_trainer(&self, trainer: Trainer) -> Result<(), sqlx::Error> {
        let result = sqlx::query("UPDATE trainers SET name = $1, birthdate = $2, sex = $3, contact_email = $4, phone_number = $5, img_url = $6 WHERE id = $7")
            .bind(trainer.name).bind(trainer.birthdate).bind(trainer.sex).bind(trainer.contact_email).bind(trainer.phone_number).bind(trainer.img_url).bind(trainer.id).execute(&self.pool).await;

        match result {
            Ok(_) => {
                info!("Success");
                Ok(())
            }
            Err(e) => {
                error!("error update_trainer: {}", e);
                Err(e)
            }
        }
    }
}
