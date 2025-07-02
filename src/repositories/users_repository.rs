use crate::entities::user::User;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait UsersRepository: Send + Sync {
    async fn get_user(&self, id: Uuid) -> Result<Option<User>, sqlx::Error>;

    async fn create_user(&self, user: User) -> Result<(), sqlx::Error>;
}
