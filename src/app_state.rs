use sqlx::PgPool;
use std::sync::Arc;
use tracing::info;

use crate::{
    infra::db::{
        dogs_repository::PgDogsRepository, trainers_repository::PgTrainersRepository,
        users_repository::PgUsersRepository,
    },
    use_cases::{
        dogs_service::DogsService, trainers_service::TrainersService, users_service::UsersService,
    },
};

#[derive(Clone)]
pub struct AppState {
    pub dogs_service: Arc<DogsService<PgDogsRepository>>,
    pub trainers_service: Arc<TrainersService<PgTrainersRepository>>,
    pub users_service: Arc<UsersService<PgUsersRepository>>,
}

impl AppState {
    pub async fn build(pool: PgPool) -> Self {
        let dogs_repo = PgDogsRepository { pool: pool.clone() };
        let dogs_service = Arc::new(DogsService::new(dogs_repo));
        info!("DogsService initialized.");

        let trainers_repo = PgTrainersRepository { pool: pool.clone() };
        let trainers_service = Arc::new(TrainersService::new(trainers_repo));
        info!("TrainersService initialized.");

        let users_repo = PgUsersRepository { pool };
        let users_service = Arc::new(UsersService::new(users_repo));
        info!("TrainersService initialized.");
        AppState {
            dogs_service,
            trainers_service,
            users_service,
        }
    }
}
