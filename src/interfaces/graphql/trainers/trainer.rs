use std::sync::Arc;

use async_graphql::{Context, Error, InputObject};

use crate::{
    infra::db::trainer_repository::PgTrainersRepository,
    interfaces::graphql::shared::uuid::GraphQLUuid, use_cases::trainers_service::TrainersService,
};

use super::{trainer_mapper, trainers_types::TrainerGQL};

#[derive(InputObject)]
pub struct TrainerInput {
    pub id: GraphQLUuid,
}

pub async fn trainer(ctx: &Context<'_>, input: TrainerInput) -> Result<TrainerGQL, Error> {
    let trainers_service = ctx
        .data::<Arc<TrainersService<PgTrainersRepository>>>()
        .map_err(|_| Error::new("TrainersService not found in context"))?;

    let trainer = trainers_service
        .get_trainer(input.id.0)
        .await
        .map_err(|e| Error::new(format!("An Error occured during reading trainer: {}", e)))?;

    match trainer {
        Some(t) => Ok(trainer_mapper::map_trainer_to_gql(t)),
        None => Err(Error::new("Trainer not found")),
    }
}
