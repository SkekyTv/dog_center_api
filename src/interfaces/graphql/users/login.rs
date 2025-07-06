use std::sync::Arc;

use async_graphql::{Context, Error, InputObject};

use crate::{
    infra::db::users_repository::PgUsersRepository,
    use_cases::{
        jwt_service::JwtServiceTrait,
        users_service::{LoginInput, UsersService},
    },
};

use super::{authorize_mapper::authorize_mapper, authorize_type::AuthorizeGQL};

#[derive(InputObject)]
#[graphql(name = "LoginInput")]
pub struct LoginInputGQL {
    pub email: String,
    pub pdw: String,
}

pub async fn login(ctx: &Context<'_>, input: LoginInputGQL) -> Result<AuthorizeGQL, Error> {
    let users_service = ctx
        .data::<Arc<UsersService<PgUsersRepository, dyn JwtServiceTrait>>>()
        .map_err(|_| Error::new("UsersService not found in context"))?;

    let token_result = users_service
        .login(LoginInput {
            email: input.email,
            pdw: input.pdw,
        })
        .await;

    match token_result {
        Ok(auth) => Ok(authorize_mapper(auth)),
        Err(e) => Err(Error::new(format!("Error login: {}", e))),
    }
}
