use crate::entities::user::Authorize;

use super::authorize_type::AuthorizeGQL;

pub fn authorize_mapper(auth: Authorize) -> AuthorizeGQL {
    AuthorizeGQL { token: auth.token }
}
