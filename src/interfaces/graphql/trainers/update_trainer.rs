use std::sync::Arc;

use async_graphql::{Context, Error, InputObject};
use garde::Validate;

use crate::{
    infra::db::trainers_repository::PgTrainersRepository,
    interfaces::graphql::shared::{graphql_date_time::GraphQLDateTime, uuid::GraphQLUuid},
    shared::types::sex::Sex,
    use_cases::trainers_service::{TrainersService, UpdateTrainerInput},
};

use super::{trainer_mapper, trainers_types::TrainerGQL};

#[derive(InputObject, Validate)]
#[graphql(name = "UpdateTrainerInput")]
#[garde(allow_unvalidated)]
pub struct GqlUpdateTrainerInput {
    pub id: GraphQLUuid,
    #[garde(length(min = 1, max = 100))]
    pub name: Option<String>,
    pub birthdate: Option<Option<GraphQLDateTime>>,

    pub sex: Option<Sex>,
    #[garde(email)]
    pub contact_email: Option<Option<String>>,

    #[garde(phone_number)]
    pub phone_number: Option<Option<String>>,
}

pub async fn update_trainer(
    ctx: &Context<'_>,
    input: GqlUpdateTrainerInput,
) -> Result<TrainerGQL, Error> {
    // Validation avec garde
    if let Err(validation_errors) = input.validate() {
        let error_messages: Vec<String> = validation_errors
            .iter()
            .map(|(field, errors)| format!("{}: {}", field, errors))
            .collect();
        return Err(Error::new(format!(
            "Validation failed: {}",
            error_messages.join("; ")
        )));
    }
    let trainers_service = ctx
        .data::<Arc<TrainersService<PgTrainersRepository>>>()
        .map_err(|_| Error::new("TrainersService not found in context"))?;

    let updated = trainers_service
        .update_trainer(UpdateTrainerInput {
            id: input.id.0,
            name: input.name,
            sex: input.sex,
            birthdate: input.birthdate.map(|opt| opt.map(|dt| dt.0)),
            contact_email: input.contact_email,
            phone_number: input.phone_number,
        })
        .await
        .map_err(|e| Error::new(format!("Error updating trainer: {}", e)))?;

    Ok(trainer_mapper::map_trainer_to_gql(updated))
}
