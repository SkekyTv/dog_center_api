use crate::{
    entities::trainers::Trainer,
    interfaces::graphql::shared::{graphql_date_time::GraphQLDateTime, uuid::GraphQLUuid},
};

use super::trainers_types::TrainerGQL;

pub fn map_trainer_to_gql(trainer: Trainer) -> TrainerGQL {
    TrainerGQL {
        id: GraphQLUuid(trainer.id),
        name: trainer.name,
        birthdate: trainer.birthdate.map(GraphQLDateTime),
        phone_number: trainer.phone_number,
        contact_email: trainer.contact_email,
        sex: trainer.sex,
    }
}
