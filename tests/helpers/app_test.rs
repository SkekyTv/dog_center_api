use super::db::setup_test_postgres;
use dotenv::dotenv;

use dog_center_api::app::run_app;
use tokio::time::{Duration, sleep};
use tracing::info;

pub async fn set_up_app_test() -> String {
    dotenv().ok();
    info!("Env var loaded.");
    let pool = setup_test_postgres().await;

    tokio::spawn(async move {
        if let Err(e) = run_app(pool).await {
            eprintln!("Test app failed to run: {:?}", e);
        }
    });

    // Étape 4 : Retourner l'URL de l'application
    let app_url = "http://127.0.0.1:8080".to_string();

    wait_for_server(app_url.as_str(), 30).await;
    info!("Test app started at {}", app_url);
    app_url
}

async fn wait_for_server(url: &str, timeout_secs: u64) {
    let mut attempts = 0;
    let max_attempts = timeout_secs;
    while attempts < max_attempts {
        if reqwest::get(url).await.is_ok() {
            return;
        }
        attempts += 1;
        sleep(Duration::from_secs(1)).await;
    }
    panic!("Server did not become ready in {} seconds", timeout_secs);
}
