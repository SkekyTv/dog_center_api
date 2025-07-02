use std::sync::Arc;

use async_graphql::{Context, Error, InputObject};
use garde::Validate;

use crate::{
    infra::db::dogs_repository::PgDogsRepository,
    interfaces::graphql::shared::graphql_date_time::GraphQLDateTime,
    shared::types::sex::Sex,
    shared::validators::option_vec_i32_validator::create_vec_range_validator,
    use_cases::dogs_service::{CreateDogInput, DogsService},
};

use super::{dog_mapper, dogs_types::DogGQL};

#[derive(InputObject, Validate)]
#[garde(allow_unvalidated)]
pub struct RegisterDogInput {
    #[garde(length(min = 1, max = 100))]
    pub name: String,
    pub birthdate: Option<GraphQLDateTime>,
    pub races: Option<Vec<String>>,
    #[garde(custom(create_vec_range_validator(-50, 50)))]
    pub weight: Option<Vec<i32>>,
    pub img_url: Option<String>,
    pub sex: Sex,
    pub icad_id: Option<String>,
}

pub async fn register_dog(ctx: &Context<'_>, input: RegisterDogInput) -> Result<DogGQL, Error> {
    let dogs_service = ctx
        .data::<Arc<DogsService<PgDogsRepository>>>()
        .map_err(|_| Error::new("DogsService not found in context"))?;

    let new_dog = dogs_service
        .create_dog(CreateDogInput {
            name: input.name,
            birthdate: input.birthdate.map(|dt| dt.0),
            races: input.races.unwrap_or_default(),
            weight: input.weight.unwrap_or_default(),
            sex: input.sex,
            icad_id: input.icad_id,
        })
        .await
        .map_err(|e| Error::new(format!("Error creating dog: {}", e)))?;

    Ok(dog_mapper::map_dog_to_gql(new_dog))
}
