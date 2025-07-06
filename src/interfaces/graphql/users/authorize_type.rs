use async_graphql::SimpleObject;

#[derive(SimpleObject)]
pub struct AuthorizeGQL {
    pub token: String,
}
