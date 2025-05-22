use sqlx::pool::PoolOptions;
use sqlx::{Pool, Postgres};
use std::time::Duration;
use testcontainers::core::ContainerPort;
use testcontainers::runners::AsyncRunner;
use testcontainers::{ContainerAsync, GenericImage, ImageExt};

pub struct PgSetup {
    pub pool: Pool<Postgres>,
    pub _container: ContainerAsync<GenericImage>,
}
pub async fn setup_test_postgres() -> PgSetup {
    // Configuration de l'image PostgreSQL
    let db_user = std::env::var("POSTGRES_USER").expect("POSTGRES_USER must be set");
    let db_pdw = std::env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD must be set");
    let db_name = std::env::var("POSTGRES_DATABASE").expect("POSTGRES_DATABASE must be set");
    let container_port = ContainerPort::from(5432);
    let container = GenericImage::new("postgres", "latest")
        .with_exposed_port(container_port)
        .with_env_var("POSTGRES_USER", &db_user)
        .with_env_var("POSTGRES_PASSWORD", &db_pdw)
        .with_env_var("POSTGRES_DB", &db_name)
        .with_env_var("POSTGRES_HOST_AUTH_METHOD", "trust")
        .start()
        .await
        .unwrap();

    // Récupérer le port exposé
    let host = container.get_host().await.unwrap();
    let host_port = container.get_host_port_ipv4(5432).await.unwrap();

    // Construire la chaîne de connexion
    let connection_string = format!(
        "postgres://{}:{}@{}:{}/{}",
        db_user, db_pdw, host, host_port, db_name
    );

    tokio::time::sleep(Duration::from_secs(5)).await;

    // Créer un pool SQLx
    let pool = PoolOptions::<Postgres>::new()
        .max_connections(30) // Définir le nombre maximum de connexions
        .min_connections(5)
        .connect(&connection_string)
        .await
        .expect("Failed to connect to PostgreSQL");

    // let mut tries = 0;
    // let pool = loop {
    //     match PoolOptions::<Postgres>::new()
    //         .max_connections(30)
    //         .min_connections(5)
    //         .connect(&connection_string)
    //         .await
    //     {
    //         Ok(pool) => break pool,
    //         Err(_e) if tries < 10 => {
    //             tries += 1;
    //             tokio::time::sleep(Duration::from_secs(1)).await;
    //         }
    //         Err(e) => panic!("Failed to connect to PostgreSQL: {}", e),
    //     }
    // };
    // Appliquer les migrations (si nécessaire)
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    tokio::time::sleep(Duration::from_secs(5)).await;

    PgSetup {
        pool,
        _container: container,
    }
}
