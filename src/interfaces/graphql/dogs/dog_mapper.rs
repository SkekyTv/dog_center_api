use crate::{
    entities::dogs::Dog,
    interfaces::graphql::shared::{graphql_date_time::GraphQLDateTime, uuid::GraphQLUuid},
};

use super::dogs_types::DogGQL;

pub fn map_dog_to_gql(dog: Dog) -> DogGQL {
    println!("mapper gql: {:?}", dog);
    DogGQL {
        id: GraphQLUuid(dog.id),
        name: dog.name,
        birthdate: dog.birthdate.map(GraphQLDateTime),
        races: dog.races,
        weight: dog.weight,
        img_url: dog.img_url,
        sex: dog.sex,
        icad_id: dog.icad_id,
        desactivation_status: match dog.desactivated_at {
            Some(_) => true,
            None => false,
        },
    }
}

#[cfg(test)]
mod tests {
    use async_graphql::{ScalarType, Value};
    use chrono::Utc;

    use crate::shared::types::sex::Sex;

    use super::*;

    #[test]
    fn test_dog_constructor_minimal_params() {
        let dog = Dog::new("pupuce".to_string(), Sex::M, None, [].to_vec(), None, None);

        let gql_dog = map_dog_to_gql(dog.clone());

        assert_eq!(gql_dog.id.to_value(), Value::String(dog.id.to_string()));

        assert_eq!(gql_dog.name, dog.name);
        assert_eq!(gql_dog.sex, Sex::M);
        assert!(gql_dog.races.is_empty());
        assert_eq!(gql_dog.birthdate, None);
        assert_eq!(gql_dog.weight, None);
        assert_eq!(gql_dog.icad_id, None);
        assert_eq!(gql_dog.desactivation_status, false)
    }

    #[test]
    fn test_dog_constructor_full_params() {
        let dog = Dog::new(
            "pupuce".to_string(),
            Sex::M,
            Some(Utc::now()),
            ["staff".to_string()].to_vec(),
            Some(100),
            Some("icad-fake-id".to_string()),
        );

        let gql_dog = map_dog_to_gql(dog.clone());

        assert_eq!(gql_dog.id.to_value(), Value::String(dog.id.to_string()));

        assert_eq!(gql_dog.name, dog.name);
        assert_eq!(gql_dog.sex, Sex::M);
        assert_eq!(gql_dog.races, ["staff".to_string()].to_vec());
        assert_eq!(gql_dog.birthdate, dog.birthdate.map(GraphQLDateTime));
        assert_eq!(gql_dog.weight, Some(100));
        assert_eq!(gql_dog.icad_id, Some("icad-fake-id".to_string()));
        assert_eq!(gql_dog.desactivation_status, false)
    }

    #[test]
    fn test_desactivated_dog() {
        let dog = Dog::new(
            "pupuce".to_string(),
            Sex::M,
            Some(Utc::now()),
            ["staff".to_string()].to_vec(),
            Some(100),
            Some("icad-fake-id".to_string()),
        );
        let desactivated_dog = dog.toggle_activation_status();

        let gql_dog = map_dog_to_gql(desactivated_dog.clone());

        assert_eq!(
            gql_dog.id.to_value(),
            Value::String(desactivated_dog.id.to_string())
        );

        assert_eq!(gql_dog.name, desactivated_dog.name);
        assert_eq!(gql_dog.sex, Sex::M);
        assert_eq!(gql_dog.races, ["staff".to_string()].to_vec());
        assert_eq!(gql_dog.birthdate, dog.birthdate.map(GraphQLDateTime));
        assert_eq!(gql_dog.weight, Some(100));
        assert_eq!(gql_dog.icad_id, Some("icad-fake-id".to_string()));
        assert_eq!(gql_dog.desactivation_status, true)
    }
}
