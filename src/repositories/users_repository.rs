use crate::entities::user::User;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait UsersRepository: Send + Sync {
    async fn get_user_by_id(&self, id: Uuid) -> Result<Option<User>, sqlx::Error>;

    async fn get_user_by_email(&self, email: String) -> Result<Option<User>, sqlx::Error>;

    async fn create_user(&self, user: User) -> Result<(), sqlx::Error>;
}
