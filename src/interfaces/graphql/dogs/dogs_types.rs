use async_graphql::SimpleObject;

use crate::{
    interfaces::graphql::shared::{
        cursor_pagination::PageInfo, graphql_date_time::GraphQLDateTime, uuid::GraphQLUuid,
    },
    shared::types::sex::Sex,
};

#[derive(SimpleObject)]
pub struct DogGQL {
    pub id: GraphQLUuid,
    pub name: String,
    pub birthdate: Option<GraphQLDateTime>,
    pub races: Vec<String>,
    pub weight: Option<i32>,
    pub img_url: Option<String>,
    pub sex: Sex,
    pub icad_id: Option<String>,
    pub desactivation_status: bool,
}

#[derive(SimpleObject)]
pub struct DogEdge {
    pub cursor: String,
    pub node: DogGQL,
}

#[derive(SimpleObject)]
pub struct DogConnection {
    pub edges: Vec<DogEdge>,
    pub page_info: PageInfo,
}
