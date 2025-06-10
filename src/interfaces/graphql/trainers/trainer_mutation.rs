use async_graphql::{Context, Error, Object};

use super::{
    register_trainer::{RegisterTrainerInput, register_trainer},
    trainers_types::TrainerGQL,
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
}

impl Default for TrainerMutation {
    fn default() -> Self {
        TrainerMutation
    }
}
