use std::sync::Arc;

use crate::use_cases::jwt_service::JwtServiceTrait;
use async_graphql::{Context, Error, Guard, Result};

pub struct AuthGuard;

#[async_trait::async_trait]
impl Guard for AuthGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        let jwt_service = ctx
            .data::<Arc<dyn JwtServiceTrait>>()
            .map_err(|_| Error::new("JwtService not found"))?;

        let auth_header = ctx
            .data_opt::<axum::http::HeaderMap>()
            .and_then(|headers| headers.get("authorization"))
            .and_then(|v| v.to_str().ok());

        let token = match auth_header {
            Some(header) if header.starts_with("Bearer ") => &header[7..],
            _ => return Err(Error::new("Authorization header missing or invalid")),
        };

        let claims = jwt_service
            .decode(token)
            .map_err(|_| Error::new("Invalid or expired token"))?;

        ctx.insert();

        Ok(())
    }
}
