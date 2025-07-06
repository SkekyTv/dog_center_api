use sqlx::PgPool;
use std::sync::Arc;
use tracing::info;

use crate::{
    infra::db::{
        dogs_repository::PgDogsRepository, trainers_repository::PgTrainersRepository,
        users_repository::PgUsersRepository,
    },
    use_cases::{
        dogs_service::DogsService,
        jwt_service::{JwtService, JwtServiceTrait},
        trainers_service::TrainersService,
        users_service::UsersService,
    },
};

#[derive(Clone)]
pub struct AppState {
    pub dogs_service: Arc<DogsService<PgDogsRepository>>,
    pub trainers_service: Arc<TrainersService<PgTrainersRepository>>,
    pub users_service: Arc<UsersService<PgUsersRepository, dyn JwtServiceTrait>>,
    pub jwt_service: Arc<dyn JwtServiceTrait>,
}

impl AppState {
    pub async fn build(pool: PgPool) -> Self {
        let dogs_repo = PgDogsRepository { pool: pool.clone() };
        let dogs_service = Arc::new(DogsService::new(dogs_repo));
        info!("DogsService initialized.");

        let trainers_repo = PgTrainersRepository { pool: pool.clone() };
        let trainers_service = Arc::new(TrainersService::new(trainers_repo));
        info!("TrainersService initialized.");

        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let jwt_service: Arc<dyn JwtServiceTrait> = Arc::new(JwtService::new(jwt_secret));
        info!("JwtService initialized.");

        let users_repo = PgUsersRepository { pool };
        let users_service = Arc::new(UsersService::new(users_repo, jwt_service.clone()));
        info!("UsersService initialized.");

        AppState {
            dogs_service,
            trainers_service,
            users_service,
            jwt_service,
        }
    }
}
