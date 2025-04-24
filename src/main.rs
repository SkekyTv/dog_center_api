use std::net::SocketAddr;

use axum::{Json, Router, response::IntoResponse, routing::get};
use interfaces::http::dogs::routes::dogs_routes;
use listenfd::ListenFd;
use tokio::net::TcpListener as TokioTcpListener;

use dotenv::dotenv;

mod db;
use db::init_fb_from_env;

mod app_state;
use app_state::AppState;
use tracing_subscriber::fmt;

mod entities;
mod infra;
mod interfaces;
mod repositories;
mod shared;
mod use_cases;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    fmt()
        .with_env_filter("sqlx=debug") // Log SQLx en mode debug
        .init();
    let pool = init_fb_from_env().await?;

    let state = AppState::build(pool).await;

    let app = Router::new()
        .route("/api/healthcheck", get(health_check_handler))
        .nest("/api", dogs_routes())
        .with_state(state);

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
