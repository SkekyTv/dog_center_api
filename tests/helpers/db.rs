use dog_center_api::db;
use sqlx::{Pool, Postgres};
use testcontainers::{ImageExt, runners::AsyncRunner};
use testcontainers_modules::postgres;
use tracing::{error, info};

pub async fn setup_test_postgres() -> Pool<Postgres> {
    let db_user = std::env::var("POSTGRES_USER").expect("POSTGRES_USER must be set");
    let db_pdw = std::env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD must be set");
    let db_name = std::env::var("POSTGRES_DATABASE").expect("POSTGRES_DATABASE must be set");

    let os = std::env::var("OS").expect("OS should be set to configure docker");
    let network = match os.as_str() {
        "mac" => "bridge",
        "windows" => "bridge",
        "linux" => "host",
        _ => panic!("Unsuportedd OS: {}", os),
    };

    let _container = match postgres::Postgres::default()
        .with_network(network)
        .with_env_var("POSTGRES_USER", db_user)
        .with_env_var("POSTGRES_PASSWORD", db_pdw)
        .with_env_var("POSTGRES_DB", db_name)
        .start()
        .await
    {
        Ok(container) => {
            info!("docker start");
            container
        }
        Err(e) => {
            error!("error docker {}", e);
            panic!("Failed to create docker container")
        }
    };

    // match container.get_host_port_ipv4(5432).await {
    //     Ok(port) => {
    //         info!("PostgreSQL is exposed on port {}", port);
    //         port
    //     }
    //     Err(_) => {
    //         error!("Failed to get exposed port for PostgreSQL.");
    //         panic!("PostgreSQL port mapping failed");
    //     }
    // };
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
