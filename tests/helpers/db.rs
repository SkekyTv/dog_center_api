use dog_center_api::db;
use sqlx::{Pool, Postgres};
use testcontainers::runners::AsyncRunner;
use testcontainers_modules::postgres;
use tracing::{error, info};

pub async fn setup_test_postgres() -> Pool<Postgres> {
    let container = match postgres::Postgres::default().start().await {
        Ok(container) => {
            info!("docker start");
            container
        }
        Err(e) => {
            error!("error docker {}", e);
            panic!("Failed to create docker container")
        }
    };

    match container.get_host_port_ipv4(5432).await {
        Ok(port) => {
            info!("PostgreSQL is exposed on port {}", port);
            port
        }
        Err(_) => {
            error!("Failed to get exposed port for PostgreSQL.");
            panic!("PostgreSQL port mapping failed");
        }
    };
    let connection_string = db::generate_db_url_from_env();

    // Créer un pool Postgres SQLx
    let pool = sqlx::PgPool::connect(&connection_string)
        .await
        .expect("Failed to connect to PostgreSQL");

    info!("Test pool started.");

    // Exécuter les migrations si nécessaire
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    pool
}
