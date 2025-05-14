use sqlx::PgPool;
use std::sync::Arc;
use tracing::info;

use crate::{infra::db::dogs_repository::PgDogsRepository, use_cases::dogs_service::DogsService};

#[derive(Clone)]
pub struct AppState {
    pub dogs_service: Arc<DogsService<PgDogsRepository>>,
}

impl AppState {
    pub async fn build(pool: PgPool) -> Self {
        let dogs_repo = PgDogsRepository { pool };
        let dogs_service = Arc::new(DogsService::new(dogs_repo));

        info!("DogsService initialized.");

        AppState { dogs_service }
    }
}
