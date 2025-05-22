use std::net::SocketAddr;

use super::db::setup_test_postgres;
use dotenv::dotenv;

use dog_center_api::app::run_app;
use sqlx::{Pool, Postgres};
use testcontainers::{ContainerAsync, GenericImage};
use tokio::time::{Duration, sleep};

use tokio::net::TcpListener as TokioTcpListener;

use tracing::info;

pub struct TestSetup {
    pub app_url: String,
    pub db_pool: Pool<Postgres>,
    pub _container: ContainerAsync<GenericImage>,
}

impl Drop for TestSetup {
    fn drop(&mut self) {
        info!("Dropping TestSetup, stopping container")
    }
}

pub async fn set_up_app_test() -> TestSetup {
    dotenv().ok();
    info!("Env var loaded.");
    let pg_setup = setup_test_postgres().await;
    // Bind a random free port
    // === REST listener ===
    let rest_host = "127.0.0.1:0".to_string();
    let rest_addr: SocketAddr = rest_host.parse().unwrap();
    let rest_tokio_listener = TokioTcpListener::bind(rest_addr).await.unwrap();
    let rest_url = format!("http://{}", rest_tokio_listener.local_addr().unwrap());

    // === GraphQL listener ===
    let graphql_host = "127.0.0.1:0".to_string();
    let addr: SocketAddr = graphql_host.parse().unwrap();
    let graphql_tokio_listener = TokioTcpListener::bind(addr).await.unwrap();
    let graphql_url = format!("http://{}", graphql_tokio_listener.local_addr().unwrap());

    let app_pool = pg_setup.pool.clone();
    tokio::spawn(async move {
        if let Err(e) = run_app(app_pool, rest_tokio_listener, graphql_tokio_listener).await {
            eprintln!("Test app failed to run: {:?}", e);
        }
    });

    wait_for_server(&graphql_url.as_str(), 30).await;
    info!("Test graphql app started at {}", graphql_url);
    info!("Test rest app started at {}", rest_url);

    TestSetup {
        app_url: graphql_url,
        db_pool: pg_setup.pool,
        _container: pg_setup._container,
    }
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
