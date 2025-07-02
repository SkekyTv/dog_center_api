use async_graphql::SimpleObject;

use crate::interfaces::graphql::shared::uuid::GraphQLUuid;

#[derive(SimpleObject)]
pub struct UserGQL {
    pub id: GraphQLUuid,
    pub email: String,
}
