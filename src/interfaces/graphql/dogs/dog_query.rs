use async_graphql::{Context, Error, Object};

use super::{
    dog::{DogInput, dog},
    dogs::{DogsInput, dogs},
    dogs_types::{DogConnection, DogGQL},
};

pub struct DogQuery;

#[Object]
impl DogQuery {
    pub async fn dog(&self, ctx: &Context<'_>, input: DogInput) -> Result<DogGQL, Error> {
        dog(ctx, input).await
    }

    pub async fn dogs(&self, ctx: &Context<'_>, input: DogsInput) -> Result<DogConnection, Error> {
        dogs(ctx, input).await
    }
}

impl Default for DogQuery {
    fn default() -> Self {
        DogQuery
    }
}
