use async_graphql::{Context, Error, Object};

use super::{
    trainer::{TrainerInput, trainer},
    trainers_types::TrainerGQL,
};

pub struct TrainerQuery;

#[Object]
impl TrainerQuery {
    async fn trainer(&self, ctx: &Context<'_>, input: TrainerInput) -> Result<TrainerGQL, Error> {
        trainer(ctx, input).await
    }
}

impl Default for TrainerQuery {
    fn default() -> Self {
        TrainerQuery
    }
}
