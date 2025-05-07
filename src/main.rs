use dotenv::dotenv;
use tracing::{error, info};
use tracing_subscriber::fmt;

mod adapters;
mod app_state;
mod db;
mod entities;
mod infra;
mod interfaces;
mod repositories;
mod shared;
mod use_cases;

use adapters::{graphql_adapter, http_adapter};
use app_state::AppState;
use db::init_fb_from_env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    info!("Env var loaded.");

    fmt().with_max_level(tracing::Level::INFO).init();
    info!("Logging started.");

    let pool = match init_fb_from_env().await {
        Ok(pool) => {
            info!("Db connected.");
            pool
        }
        Err(e) => {
            error!("Db fail to connect.");
            return Err(e);
        }
    };

    info!("Starting AppState build.");
    let state = AppState::build(pool).await;
    info!("AppState builded.");

    // Exécuter les serveurs HTTP et GraphQL en parallèle
    info!("Starting servers.");
    match tokio::try_join!(
        http_adapter::start_http_server(state.clone()),
        graphql_adapter::start_graphql_server(state.clone())
    ) {
        Ok(_) => info!("HTTP and GraphQL server started."),
        Err(e) => error!("Fail to start HTTP and GraphQL server: {}", e),
    };

    Ok(())
}
