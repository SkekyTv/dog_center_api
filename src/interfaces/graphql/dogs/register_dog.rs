use std::sync::Arc;

use async_graphql::{Context, Error, InputObject};

use crate::{
    entities::dogs::Dog, infra::db::dogs_repository::PgDogsRepository,
    interfaces::graphql::shared::graphql_date_time::GraphQLDateTime, shared::types::sex::Sex,
    use_cases::dogs_service::DogsService,
};

use super::{dog_mapper, dogs_types::DogGQL};

#[derive(InputObject)]
pub struct RegisterDogInput {
    pub name: String,
    pub birthdate: Option<GraphQLDateTime>,
    pub races: Vec<String>,
    pub weight: Option<i32>,
    pub img_url: Option<String>,
    pub sex: Sex,
    pub icad_id: Option<String>,
}

pub async fn register_dog(ctx: &Context<'_>, input: RegisterDogInput) -> Result<DogGQL, Error> {
    let dogs_service = ctx
        .data::<Arc<DogsService<PgDogsRepository>>>()
        .map_err(|_| Error::new("DogsService not found in context"))?;

    let new_dog = Dog::new(
        input.name,
        input.sex,
        input.birthdate.map(|dt| dt.0),
        input.races,
        input.weight,
        input.icad_id,
    );

    dogs_service
        .create_dog(new_dog.clone())
        .await
        .map_err(|e| Error::new(format!("Error creating dog: {}", e)))?;

    Ok(dog_mapper::map_dog_to_gql(new_dog))
}
