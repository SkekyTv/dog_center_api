use dotenv::dotenv;
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
    fmt()
        .with_env_filter("sqlx=debug") // Log SQLx en mode debug
        .init();

    let pool = init_fb_from_env().await?;
    let state = AppState::build(pool).await;

    // Exécuter les serveurs HTTP et GraphQL en parallèle
    tokio::try_join!(
        http_adapter::start_http_server(state.clone()),
        graphql_adapter::start_graphql_server(state.clone())
    )?;

    Ok(())
}
