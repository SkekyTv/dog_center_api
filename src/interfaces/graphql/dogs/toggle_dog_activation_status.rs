use std::sync::Arc;

use async_graphql::{Context, Error, InputObject};

use crate::{
    infra::db::dogs_repository::PgDogsRepository, interfaces::graphql::shared::uuid::GraphQLUuid,
    use_cases::dogs_service::DogsService,
};

use super::{dog_mapper, dogs_types::DogGQL};

#[derive(InputObject)]
#[graphql(name = "ToggleDogActivationStatusInput")]
pub struct GqlToggleDogActivationStatusInput {
    pub id: GraphQLUuid,
}

pub async fn toggle_dog_activation_status(
    ctx: &Context<'_>,
    input: GqlToggleDogActivationStatusInput,
) -> Result<DogGQL, Error> {
    let dogs_service = ctx
        .data::<Arc<DogsService<PgDogsRepository>>>()
        .map_err(|_| Error::new("DogsService not found in context"))?;

    let toogle_dog = dogs_service
        .toggle_activation_status(input.id.0)
        .await
        .map_err(|e| Error::new(format!("Error toggling dog activation status: {}", e)))?;

    Ok(dog_mapper::map_dog_to_gql(toogle_dog))
}
