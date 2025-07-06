use crate::{entities::user::User, interfaces::graphql::shared::uuid::GraphQLUuid};

use super::user_types::UserGQL;

pub fn user_mapper(user: User) -> UserGQL {
    UserGQL {
        id: GraphQLUuid(user.id),
        email: user.email,
    }
}
