use async_graphql::{Context, InputObject, parser::Error};

use super::user_types::UserGQL;

#[derive(InputObject)]
pub struct SignUpInput {
    pub email: String,
    pub pdw: String,
}

pub async fn sign_up(ctx: &Context<'_>, input: SignUpInput) -> Result<UserGQL, Error> {
    todo!()
}
