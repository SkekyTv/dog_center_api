use async_graphql::SimpleObject;

use crate::{
    interfaces::graphql::shared::{
        cursor_pagination::PageInfo, graphql_date_time::GraphQLDateTime, uuid::GraphQLUuid,
    },
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

#[derive(SimpleObject)]
pub struct TrainerEdge {
    pub cursor: String,
    pub node: TrainerGQL,
}

#[derive(SimpleObject)]
pub struct TrainerConnection {
    pub edges: Vec<TrainerEdge>,
    pub page_info: PageInfo,
}
