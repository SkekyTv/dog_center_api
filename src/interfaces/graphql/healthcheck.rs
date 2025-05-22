use async_graphql::Object;

pub struct HealthCheckQuery;

#[Object]
impl HealthCheckQuery {
    async fn health_check(&self) -> &str {
        "GraphQL API is running"
    }
}

impl Default for HealthCheckQuery {
    fn default() -> Self {
        HealthCheckQuery
    }
}
