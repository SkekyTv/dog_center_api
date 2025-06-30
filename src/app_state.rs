use sqlx::PgPool;
use std::sync::Arc;
use tracing::info;

use crate::{
    infra::db::{dogs_repository::PgDogsRepository, trainers_repository::PgTrainersRepository},
    use_cases::{dogs_service::DogsService, trainers_service::TrainersService},
};

#[derive(Clone)]
pub struct AppState {
    pub dogs_service: Arc<DogsService<PgDogsRepository>>,
    pub trainers_service: Arc<TrainersService<PgTrainersRepository>>,
}

impl AppState {
    pub async fn build(pool: PgPool) -> Self {
        let dogs_repo = PgDogsRepository { pool: pool.clone() };
        let dogs_service = Arc::new(DogsService::new(dogs_repo));
        info!("DogsService initialized.");

        let trainers_repo = PgTrainersRepository { pool };
        let trainers_service = Arc::new(TrainersService::new(trainers_repo));
        info!("TrainersService initialized.");

        AppState {
            dogs_service,
            trainers_service,
        }
    }
}
