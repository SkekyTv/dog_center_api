use async_graphql::{Context, Error, Object};

use super::{
    trainer::{TrainerInput, trainer},
    trainers_paginated::{TrainersInput, trainers},
    trainers_types::{TrainerConnection, TrainerGQL},
};

pub struct TrainerQuery;

#[Object]
impl TrainerQuery {
    pub async fn trainer(
        &self,
        ctx: &Context<'_>,
        input: TrainerInput,
    ) -> Result<TrainerGQL, Error> {
        trainer(ctx, input).await
    }

    pub async fn trainers(
        &self,
        ctx: &Context<'_>,
        input: TrainersInput,
    ) -> Result<TrainerConnection, Error> {
        trainers(ctx, input).await
    }
}

impl Default for TrainerQuery {
    fn default() -> Self {
        TrainerQuery
    }
}
