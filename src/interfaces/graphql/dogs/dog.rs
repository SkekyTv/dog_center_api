use std::sync::Arc;

use async_graphql::{Context, Error, InputObject, Object};

use crate::{
    infra::db::dogs_repository::PgDogsRepository, interfaces::graphql::shared::uuid::GraphQLUuid,
    use_cases::dogs_service::DogsService,
};

use super::{dog_mapper, dogs_types::DogGQL};

pub struct DogQuery;

#[derive(InputObject)]
pub struct DogInput {
    pub id: GraphQLUuid,
}

#[Object]
impl DogQuery {
    pub async fn dog(&self, ctx: &Context<'_>, input: DogInput) -> Result<DogGQL, Error> {
        let dogs_service = ctx
            .data::<Arc<DogsService<PgDogsRepository>>>()
            .map_err(|_| Error::new("DogService not found in context"))?;

        let dog = match dogs_service.get_dog(input.id.0).await {
            Ok(dog) => dog,
            Err(e) => panic!("An error occured reading dog: {}", e),
        };

        match dog {
            Some(dog) => Ok(dog_mapper::map_dog_to_gql(dog)),
            None => Err(Error::new("Dog not found")),
        }
    }
}

impl Default for DogQuery {
    fn default() -> Self {
        DogQuery
    }
}
