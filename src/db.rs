use sqlx::{PgPool, postgres::PgPoolOptions};

pub fn generate_db_url_from_env() -> String {
    let db_user = std::env::var("POSTGRES_USER").expect("POSTGRES_USER must be set");
    let db_pdw = std::env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD must be set");
    let db_host = std::env::var("POSTGRES_HOST").expect("POSTGRES_HOST must be set");
    let db_name = std::env::var("POSTGRES_DATABASE").expect("POSTGRES_DATABASE must be set");
    format!("postgres://{}:{}@{}/{}", db_user, db_pdw, db_host, db_name)
}

pub async fn init_fb_from_env() -> Result<PgPool, Box<dyn std::error::Error>> {
    let db_user = std::env::var("POSTGRES_USER").expect("POSTGRES_USER must be set");
    let db_pdw = std::env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD must be set");
    let db_host = std::env::var("POSTGRES_HOST").expect("POSTGRES_HOST must be set");
    let db_name = std::env::var("POSTGRES_DATABASE").expect("POSTGRES_DATABASE must be set");
    // Connexion à la base "postgres"
    let default_conn_str = format!("postgres://{}:{}@{}/postgres", db_user, db_pdw, db_host);
    let admin_pool = PgPool::connect(&default_conn_str).await?;

    // Vérifie si la base existe, sinon la crée
    let db_exists =
        sqlx::query_scalar::<_, Option<i32>>("SELECT 1 FROM pg_database WHERE datname = $1")
            .bind(&db_name)
            .fetch_optional(&admin_pool)
            .await?
            .is_some();

    if !db_exists {
        println!("Creating database '{}'", db_name);
        sqlx::Executor::execute(
            &admin_pool,
            format!(r#"CREATE DATABASE "{}""#, db_name).as_str(),
        )
        .await?;
    } else {
        println!("Database '{}' already exists", db_name);
    }
    let postgres_url = generate_db_url_from_env();

    let app_pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&postgres_url)
        .await?;

    println!("Connected to database '{}'", db_name);

    sqlx::migrate!().run(&app_pool).await?;

    Ok(app_pool)
}
