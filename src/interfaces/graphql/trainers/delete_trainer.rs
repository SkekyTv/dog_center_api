use std::sync::Arc;

use async_graphql::{Context, Error, InputObject};

use crate::{
    infra::db::trainer_repository::PgTrainersRepository,
    interfaces::graphql::shared::uuid::GraphQLUuid, use_cases::trainers_service::TrainersService,
};

use super::{trainer_mapper, trainers_types::TrainerGQL};

#[derive(InputObject)]
#[graphql(name = "DeleteTrainerInput")]
pub struct GqlDeleteTrainerInput {
    pub id: GraphQLUuid,
}

pub async fn delete_trainer(
    ctx: &Context<'_>,
    input: GqlDeleteTrainerInput,
) -> Result<TrainerGQL, Error> {
    let trainers_service = ctx
        .data::<Arc<TrainersService<PgTrainersRepository>>>()
        .map_err(|_| Error::new("TrainersService not found in context"))?;

    let deleted = trainers_service
        .delete_trainer(input.id.0)
        .await
        .map_err(|e| Error::new(format!("Error deleting trainer: {}", e)))?;

    Ok(trainer_mapper::map_trainer_to_gql(deleted))
}
