use std::sync::Arc;

use async_graphql::{Context, Error, InputObject};

use crate::{
    infra::db::dogs_repository::PgDogsRepository,
    interfaces::graphql::shared::{graphql_date_time::GraphQLDateTime, uuid::GraphQLUuid},
    shared::types::sex::Sex,
    use_cases::dogs_service::{DogsService, UpdateDogInput},
};

use super::{dog_mapper, dogs_types::DogGQL};

#[derive(InputObject)]
#[graphql(name = "UpdateDogInput")]
pub struct GqlUpdateDogInput {
    pub id: GraphQLUuid,
    pub name: Option<String>,
    pub birthdate: Option<Option<GraphQLDateTime>>,
    pub races: Option<Vec<String>>,
    pub weight: Option<Vec<i32>>,
    pub sex: Option<Sex>,
    pub icad_id: Option<Option<String>>,
}

pub async fn update_dog(ctx: &Context<'_>, input: GqlUpdateDogInput) -> Result<DogGQL, Error> {
    let dogs_service = ctx
        .data::<Arc<DogsService<PgDogsRepository>>>()
        .map_err(|_| Error::new("DogsService not found in context"))?;

    let updated = dogs_service
        .update_dog(UpdateDogInput {
            id: input.id.0,
            name: input.name,
            sex: input.sex,
            birthdate: input.birthdate.map(|opt| opt.map(|dt| dt.0)),
            races: input.races,
            weight: input.weight,
            icad_id: input.icad_id,
        })
        .await
        .map_err(|e| Error::new(format!("Error updating dog: {}", e)))?;

    Ok(dog_mapper::map_dog_to_gql(updated))
}
