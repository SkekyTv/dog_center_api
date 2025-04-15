use std::{net::SocketAddr, sync::Arc};

use axum::{Json, Router, response::IntoResponse, routing::get};
use listenfd::ListenFd;
use sqlx::PgPool;
use tokio::net::TcpListener as TokioTcpListener;

use dotenv::dotenv;

mod db;
use db::init_fb_from_env;

pub struct AppState {
    db: PgPool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let pool = init_fb_from_env().await?;

    let app = Router::new()
        .route("/api/healthcheck", get(health_check_handler))
        .with_state(Arc::new(AppState { db: pool.clone() }));

    let listener = get_listener().await.expect("failed to bind listener");

    println!("Server listening on: {}", listener.local_addr().unwrap());
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn get_listener() -> std::io::Result<TokioTcpListener> {
    if let Some(l) = ListenFd::from_env().take_tcp_listener(0).unwrap() {
        println!("Detected systemfd - using file descriptor FD 3");
        l.set_nonblocking(true).expect("failed to unblock listener");
        TokioTcpListener::from_std(l)
    } else {
        let server_port = std::env::var("PORT").expect("PORT must be set.");
        let server_host = "0.0.0.0:".to_owned() + &server_port;
        let addr: SocketAddr = server_host.parse().unwrap();
        TokioTcpListener::bind(addr).await
    }
}

pub async fn health_check_handler() -> impl IntoResponse {
    const MESSAGE: &str = "API Services";

    let json_response = serde_json::json!({
        "status": "ok",
        "message": MESSAGE
    });

    Json(json_response)
}
