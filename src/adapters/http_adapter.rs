use crate::{app_state::AppState, interfaces::http::dogs::routes::dogs_routes};
use axum::{Router, routing::get};
use listenfd::ListenFd;
use std::net::SocketAddr;
use tokio::net::TcpListener as TokioTcpListener;
use tracing::info;

pub async fn start_http_server(
    state: AppState,
    listener: TokioTcpListener,
) -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/api/healthcheck", get(health_check_handler))
        .nest("/api", dogs_routes())
        .with_state(state.clone());

    // let listener = get_listener().await.expect("failed to bind listener");

    info!(
        "HTTP Server listening on: {}",
        listener.local_addr().unwrap()
    );
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

pub async fn get_listener() -> std::io::Result<TokioTcpListener> {
    if let Some(l) = ListenFd::from_env().take_tcp_listener(0).unwrap() {
        info!("Detected systemfd - using file descriptor FD 3");
        l.set_nonblocking(true).expect("failed to unblock listener");
        TokioTcpListener::from_std(l)
    } else {
        let server_port = std::env::var("HTTP_PORT").expect("HTTP_PORT must be set.");
        let server_host = "0.0.0.0:".to_owned() + &server_port;
        let addr: SocketAddr = server_host.parse().unwrap();
        TokioTcpListener::bind(addr).await
    }
}

async fn health_check_handler() -> impl axum::response::IntoResponse {
    const MESSAGE: &str = "API Services";

    let json_response = serde_json::json!({
        "status": "ok",
        "message": MESSAGE
    });

    axum::Json(json_response)
}
