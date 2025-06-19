use async_graphql::{Context, Error, Object};

use super::{
    delete_trainer::{GqlDeleteTrainerInput, delete_trainer},
    register_trainer::{RegisterTrainerInput, register_trainer},
    trainers_types::TrainerGQL,
    update_trainer::{GqlUpdateTrainerInput, update_trainer},
};

pub struct TrainerMutation;

#[Object]
impl TrainerMutation {
    async fn register_trainer(
        &self,
        ctx: &Context<'_>,
        input: RegisterTrainerInput,
    ) -> Result<TrainerGQL, Error> {
        register_trainer(ctx, input).await
    }

    async fn update_trainer(
        &self,
        ctx: &Context<'_>,
        input: GqlUpdateTrainerInput,
    ) -> Result<TrainerGQL, Error> {
        update_trainer(ctx, input).await
    }

    async fn delete_trainer(
        &self,
        ctx: &Context<'_>,
        input: GqlDeleteTrainerInput,
    ) -> Result<TrainerGQL, Error> {
        delete_trainer(ctx, input).await
    }
}

impl Default for TrainerMutation {
    fn default() -> Self {
        TrainerMutation
    }
}
