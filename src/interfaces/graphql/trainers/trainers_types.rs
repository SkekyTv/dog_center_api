use async_graphql::SimpleObject;

use crate::{
    interfaces::graphql::shared::{graphql_date_time::GraphQLDateTime, uuid::GraphQLUuid},
    shared::types::sex::Sex,
};

#[derive(SimpleObject)]
pub struct TrainerGQL {
    pub id: GraphQLUuid,
    pub name: String,
    pub contact_email: Option<String>,
    pub phone_number: Option<String>,
    pub birthdate: Option<GraphQLDateTime>,
    pub sex: Sex,
}
