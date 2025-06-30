use std::sync::Arc;

use async_graphql::{Context, Error, InputObject};

use crate::{
    infra::db::trainers_repository::PgTrainersRepository,
    interfaces::graphql::shared::graphql_date_time::GraphQLDateTime,
    shared::types::sex::Sex,
    use_cases::trainers_service::{CreateTrainerInput, TrainersService},
};

use super::{trainer_mapper, trainers_types::TrainerGQL};

#[derive(InputObject)]
pub struct RegisterTrainerInput {
    pub name: String,
    pub birthdate: Option<GraphQLDateTime>,
    pub sex: Sex,
    pub phone_number: Option<String>,
    pub contact_email: Option<String>,
}

pub async fn register_trainer(
    ctx: &Context<'_>,
    input: RegisterTrainerInput,
) -> Result<TrainerGQL, Error> {
    let trainers_service = ctx
        .data::<Arc<TrainersService<PgTrainersRepository>>>()
        .map_err(|_| Error::new("TrainersService not found in context."))?;

    let new_trainer = trainers_service
        .create_trainer(CreateTrainerInput {
            name: input.name,
            birthdate: input.birthdate.map(|dt| dt.0),
            sex: input.sex,
            phone_number: input.phone_number,
            contact_email: input.contact_email,
        })
        .await
        .map_err(|e| Error::new(format!("Error creating trainer: {}", e)))?;

    Ok(trainer_mapper::map_trainer_to_gql(new_trainer))
}
