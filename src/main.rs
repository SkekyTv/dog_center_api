use dog_center_api::{app::run_app, db::init_fb_from_env};
use dotenv::dotenv;
use tracing::{error, info};
use tracing_subscriber::fmt;

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

    run_app(pool).await
}
