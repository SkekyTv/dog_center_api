use crate::interfaces::graphql::dogs::dogs_types::DogGQL;
use async_graphql::{Context, Error, Object};

use super::{
    register_dog::{RegisterDogInput, register_dog},
    toggle_dog_activation_status::{
        GqlToggleDogActivationStatusInput, toggle_dog_activation_status,
    },
    update_dog::{GqlUpdateDogInput, update_dog},
};

pub struct DogMutation;

#[Object]
impl DogMutation {
    async fn register_dog(
        &self,
        ctx: &Context<'_>,
        input: RegisterDogInput,
    ) -> Result<DogGQL, Error> {
        register_dog(ctx, input).await
    }

    async fn update_dog(
        &self,
        ctx: &Context<'_>,
        input: GqlUpdateDogInput,
    ) -> Result<DogGQL, Error> {
        update_dog(ctx, input).await
    }

    async fn toggle_dog_activation_status(
        &self,
        ctx: &Context<'_>,
        input: GqlToggleDogActivationStatusInput,
    ) -> Result<DogGQL, Error> {
        toggle_dog_activation_status(ctx, input).await
    }
}

impl Default for DogMutation {
    fn default() -> Self {
        DogMutation
    }
}
