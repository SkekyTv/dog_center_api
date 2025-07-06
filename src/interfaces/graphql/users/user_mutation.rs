use async_graphql::{Context, Error, Object};

use super::{
    authorize_type::AuthorizeGQL,
    login::{LoginInputGQL, login},
    sign_up::{SignUpInputGQL, sign_up},
    user_types::UserGQL,
};

pub struct UserMutation;

#[Object]
impl UserMutation {
    async fn sign_up(&self, ctx: &Context<'_>, input: SignUpInputGQL) -> Result<UserGQL, Error> {
        sign_up(ctx, input).await
    }

    async fn login(&self, ctx: &Context<'_>, input: LoginInputGQL) -> Result<AuthorizeGQL, Error> {
        login(ctx, input).await
    }
}

impl Default for UserMutation {
    fn default() -> Self {
        UserMutation
    }
}
