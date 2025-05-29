use std::sync::Arc;

use async_graphql::{Context, Error, InputObject};

use crate::{
    infra::db::dogs_repository::PgDogsRepository,
    interfaces::graphql::shared::graphql_date_time::GraphQLDateTime,
    shared::types::sex::Sex,
    use_cases::dogs_service::{CreateDogInput, DogsService},
};

use super::{dog_mapper, dogs_types::DogGQL};

#[derive(InputObject)]
pub struct RegisterDogInput {
    pub name: String,
    pub birthdate: Option<GraphQLDateTime>,
    pub races: Option<Vec<String>>,
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
            races: match input.races {
                Some(r) => r,
                None => vec![],
            },
            weight: match input.weight {
                Some(w) => w,
                None => vec![],
            },
            sex: input.sex,
            icad_id: input.icad_id,
        })
        .await
        .map_err(|e| Error::new(format!("Error creating dog: {}", e)))?;

    Ok(dog_mapper::map_dog_to_gql(new_dog))
}
