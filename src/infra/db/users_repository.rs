use async_trait::async_trait;
use sqlx::{Error, PgPool};
use tracing::{error, info};
use uuid::Uuid;

use crate::{entities::user::User, repositories::users_repository::UsersRepository};

#[derive(Clone)]
pub struct PgUsersRepository {
    pub pool: PgPool,
}

#[async_trait]
impl UsersRepository for PgUsersRepository {
    async fn get_user_by_id(&self, id: Uuid) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT id::Uuid, email, pdw_hash FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }

    async fn get_user_by_email(&self, email: String) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT id::Uuid, email, pdw_hash FROM users WHERE email = $1")
            .bind(email)
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
                if let sqlx::Error::Database(db_err) = &e {
                    // Pour PostgreSQL, le code de violation d'unicité est "23505"
                    if db_err.code().as_deref() == Some("23505") {
                        error!("User with this email already exists");
                        return Err(Error::InvalidArgument("Email already exist".to_string()));
                    }
                }
                error!("error create_user: {}", e);
                Err(e)
            }
        }
    }
}
