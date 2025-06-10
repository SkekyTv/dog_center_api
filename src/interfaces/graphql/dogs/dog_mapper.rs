use crate::{
    entities::dogs::Dog,
    interfaces::graphql::shared::{
        graphql_date_time::GraphQLDateTime,
        measure::{Measure, Unit::G},
        uuid::GraphQLUuid,
    },
};

use super::dogs_types::DogGQL;

pub fn map_dog_to_gql(dog: Dog) -> DogGQL {
    DogGQL {
        id: GraphQLUuid(dog.id),
        name: dog.name,
        birthdate: dog.birthdate.map(GraphQLDateTime),
        races: dog.races,
        weight: dog
            .weight
            .iter()
            .map(|w| Measure { value: *w, unit: G })
            .collect(),
        img_url: dog.img_url,
        sex: dog.sex,
        icad_id: dog.icad_id,
        desactivation_status: dog.desactivated_at.is_some(),
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
        let dog = Dog::new(
            "pupuce".to_string(),
            Sex::M,
            None,
            [].to_vec(),
            vec![],
            None,
        );

        let gql_dog = map_dog_to_gql(dog.clone());

        assert_eq!(gql_dog.id.to_value(), Value::String(dog.id.to_string()));

        assert_eq!(gql_dog.name, dog.name);
        assert_eq!(gql_dog.sex, Sex::M);
        assert!(gql_dog.races.is_empty());
        assert_eq!(gql_dog.birthdate, None);
        assert_eq!(gql_dog.weight, vec![]);
        assert_eq!(gql_dog.icad_id, None);
        assert!(!gql_dog.desactivation_status)
    }

    #[test]
    fn test_dog_constructor_full_params() {
        let dog = Dog::new(
            "pupuce".to_string(),
            Sex::M,
            Some(Utc::now()),
            ["staff".to_string()].to_vec(),
            vec![100],
            Some("icad-fake-id".to_string()),
        );

        let gql_dog = map_dog_to_gql(dog.clone());

        assert_eq!(gql_dog.id.to_value(), Value::String(dog.id.to_string()));

        assert_eq!(gql_dog.name, dog.name);
        assert_eq!(gql_dog.sex, Sex::M);
        assert_eq!(gql_dog.races, ["staff".to_string()].to_vec());
        assert_eq!(gql_dog.birthdate, dog.birthdate.map(GraphQLDateTime));
        assert_eq!(
            gql_dog.weight,
            vec![Measure {
                unit: G,
                value: 100,
            }]
        );
        assert_eq!(gql_dog.icad_id, Some("icad-fake-id".to_string()));
        assert!(!gql_dog.desactivation_status)
    }

    #[test]
    fn test_desactivated_dog() {
        let dog = Dog::new(
            "pupuce".to_string(),
            Sex::M,
            Some(Utc::now()),
            ["staff".to_string()].to_vec(),
            vec![100],
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
        assert_eq!(
            gql_dog.weight,
            vec![Measure {
                unit: G,
                value: 100,
            }]
        );
        assert_eq!(gql_dog.icad_id, Some("icad-fake-id".to_string()));
        assert!(gql_dog.desactivation_status)
    }
}
