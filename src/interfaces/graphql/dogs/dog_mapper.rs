use crate::{
    entities::dogs::Dog,
    interfaces::graphql::shared::{graphql_date_time::GraphQLDateTime, uuid::GraphQLUuid},
};

use super::dogs::DogGQL;

pub fn map_dog_to_gql(dog: Dog) -> DogGQL {
    DogGQL {
        id: GraphQLUuid(dog.id),
        name: dog.name,
        birthdate: dog.birthdate.map(GraphQLDateTime),
        races: dog.races,
        weight: dog.weight,
        img_url: dog.img_url,
        sex: dog.sex,
        icad_id: dog.icad_id,
    }
}
