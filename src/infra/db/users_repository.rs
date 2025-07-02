use async_trait::async_trait;
use sqlx::PgPool;
use tracing::{error, info};
use uuid::Uuid;

use crate::{entities::user::User, repositories::users_repository::UsersRepository};

#[derive(Clone)]
pub struct PgUsersRepository {
    pub pool: PgPool,
}

#[async_trait]
impl UsersRepository for PgUsersRepository {
    async fn get_user(&self, id: Uuid) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT id::Uuid, email, pdw_hash FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }

    async fn create_user(&self, user: User) -> Result<(), sqlx::Error> {
        let result = sqlx::query("INSERT INTO users (id, email, pdw_hash) VALUES ($1, $2, $3)")
            .bind(user.id)
            .bind(user.email)
            .bind(user.pdw_hash)
            .execute(&self.pool)
            .await;

        match result {
            Ok(_) => {
                info!("Success creating user");
                Ok(())
            }
            Err(e) => {
                error!("error create_user: {}", e);
                Err(e)
            }
        }
    }
}
