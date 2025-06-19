use async_trait::async_trait;
use base64::{Engine, prelude::BASE64_STANDARD};
use sqlx::PgPool;
use tracing::{error, info};
use uuid::Uuid;

use crate::{
    entities::{
        shared::page_info::PageInfo,
        trainers::{Trainer, TrainerConnection, TrainerEdge},
    },
    repositories::trainers_repository::{ListTrainerInput, TrainersRepository},
};

#[derive(Clone)]
pub struct PgTrainersRepository {
    pub pool: PgPool,
}

#[async_trait]
impl TrainersRepository for PgTrainersRepository {
    async fn get_trainer(&self, id: Uuid) -> Result<Option<Trainer>, sqlx::Error> {
        sqlx::query_as::<_, Trainer>(
            "SELECT id::Uuid, name::Text, birthdate,  img_url, sex, contact_email, phone_number, deleted_at FROM trainers WHERE id = $1",
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

    async fn list_trainers(
        &self,
        input: ListTrainerInput,
    ) -> Result<TrainerConnection, sqlx::Error> {
        let result = sqlx::query_as::<_, Trainer>("SELECT id::Uuid, name::Text, birthdate,  img_url, sex, contact_email, phone_number, deleted_at FROM trainers WHERE ($1 IS NULL OR id > $1::uuid) ORDER BY id ASC LIMIT $2").bind(input.after_id).bind(input.first + 1).fetch_all(&self.pool).await;

        let trainers = match result {
            Ok(t) => t,
            Err(e) => {
                error!("error list_trainers: {}", e);
                return Err(e);
            }
        };

        let has_next_page = trainers.len() > input.first as usize;

        let items = trainers
            .into_iter()
            .take(input.first as usize)
            .collect::<Vec<_>>();

        let edges: Vec<TrainerEdge> = items
            .iter()
            .map(|trainer| TrainerEdge {
                cursor: BASE64_STANDARD.encode(trainer.id.to_string()),
                node: trainer.clone(),
            })
            .collect();
        let end_cursor = edges.last().map(|e| e.cursor.clone());

        Ok(TrainerConnection {
            edges,
            page_info: PageInfo {
                end_cursor,
                has_next_page,
            },
        })
    }

    async fn delete_trainer(&self, trainer: Trainer) -> Result<(), sqlx::Error> {
        let result = sqlx::query("UPDATE trainers SET deleted_at = $1 WHERE id = $2")
            .bind(trainer.deleted_at)
            .bind(trainer.id)
            .execute(&self.pool)
            .await;

        match result {
            Ok(_) => info!("Success"),
            Err(e) => error!("error toogle_activation_status : {}", e),
        }

        Ok(())
    }
}
