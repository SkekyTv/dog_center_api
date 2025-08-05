use crate::use_cases::jwt_service::JwtServiceTrait;
use async_graphql::{
    ServerResult,
    extensions::{Extension, ExtensionContext, ExtensionFactory},
};
use std::sync::Arc;

pub struct ClaimsExtension;

#[async_trait::async_trait]
impl ExtensionFactory for ClaimsExtension {
    async fn create(&self, ctx: &ExtensionContext<'_>) -> ServerResult<Box<dyn Extension>> {
        let jwt_service = ctx
            .data::<Arc<dyn JwtServiceTrait>>()
            .map_err(|_| "JwtService not found")?;

        let auth_header = ctx
            .data_opt::<axum::http::HeaderMap>()
            .and_then(|headers| headers.get("authorization"))
            .and_then(|v| v.to_str().ok());

        if let Some(header) = auth_header {
            if header.starts_with("Bearer ") {
                let token = &header[7..];
                if let Ok(claims) = jwt_service.decode(token) {
                    ctx.insert(claims);
                }
            }
        }
        Ok(Box::new(Self))
    }
}
