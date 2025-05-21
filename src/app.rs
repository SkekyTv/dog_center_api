use sqlx::{Pool, Postgres};
use tokio::net::TcpListener as TokioTcpListener;
use tracing::{error, info};

use crate::{
    adapters::{graphql_adapter, http_adapter},
    app_state::AppState,
};

pub async fn run_app(
    pool: Pool<Postgres>,
    http_listener: TokioTcpListener,
    graphql_listener: TokioTcpListener,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting AppState build.");
    let state = AppState::build(pool).await;
    info!("AppState builded.");

    // Exécuter les serveurs HTTP et GraphQL en parallèle
    info!("Starting servers.");

    // Spawn the HTTP server
    let state_http = state.clone();
    let http_server = tokio::spawn(async {
        if let Err(e) = http_adapter::start_http_server(state_http, http_listener).await {
            error!("HTTP server failed: {}", e);
        }
    });

    // Spawn the GraphQL server
    let state_graphql = state.clone();
    let graphql_server = tokio::spawn(async {
        if let Err(e) = graphql_adapter::start_graphql_server(state_graphql, graphql_listener).await
        {
            error!("GraphQL server failed: {}", e);
        }
    });

    info!("Servers started. Waiting for them to complete...");

    // Await both servers separately
    let http_result = http_server.await;
    let graphql_result = graphql_server.await;

    // Handle any errors from the tasks
    if let Err(e) = http_result {
        error!("HTTP server panicked: {:?}", e);
    }

    if let Err(e) = graphql_result {
        error!("GraphQL server panicked: {:?}", e);
    }
    Ok(())
}
