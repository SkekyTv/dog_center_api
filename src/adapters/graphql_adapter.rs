use crate::app_state::AppState;
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::response::{Html, IntoResponse};
use axum::{Router, extract::Extension, routing::get};
use listenfd::ListenFd;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener as TokioTcpListener;

// Définir la racine des requêtes GraphQL
pub struct QueryRoot;
pub struct MutationRoot;

#[async_graphql::Object]
impl QueryRoot {
    async fn health_check(&self) -> &str {
        "GraphQL API is running"
    }
}

// Construire le schéma GraphQL
fn build_schema(state: AppState) -> Schema<QueryRoot, MutationRoot, EmptySubscription> {
    let dogs_service = Arc::new(state.dogs_service.clone());
    Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(dogs_service)
        .data(state)
        .finish()
}

pub async fn start_graphql_server(state: AppState) -> Result<(), Box<dyn std::error::Error>> {
    let schema = build_schema(state);

    let app = Router::new()
        .route("/graphql", get(graphql_playground).post(graphql_handler))
        .layer(Extension(schema));

    let listener = get_listener().await.expect("failed to bind listener");

    println!(
        "GraphQL Server listening on: {}",
        listener.local_addr().unwrap()
    );
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn get_listener() -> std::io::Result<TokioTcpListener> {
    if let Some(l) = ListenFd::from_env().take_tcp_listener(1).unwrap() {
        println!("Detected systemfd - using file descriptor FD 4");
        l.set_nonblocking(true).expect("failed to unblock listener");
        TokioTcpListener::from_std(l)
    } else {
        let server_port = std::env::var("GRAPHQL_PORT").expect("GRAPHQL_PORT must be set.");
        let server_host = "0.0.0.0:".to_owned() + &server_port;
        let addr: SocketAddr = server_host.parse().unwrap();
        TokioTcpListener::bind(addr).await
    }
}

// Handler pour exécuter des requêtes GraphQL
async fn graphql_handler(
    schema: Extension<Schema<QueryRoot, EmptyMutation, EmptySubscription>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

// Handler pour GraphQL Playground
pub async fn graphql_playground() -> impl IntoResponse {
    Html(
        r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>GraphQL Playground</title>
            <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/graphql-playground-react/build/static/css/index.css" />
            <script src="https://cdn.jsdelivr.net/npm/graphql-playground-react/build/static/js/middleware.js"></script>
        </head>
        <body>
            <div id="root"></div>
            <script>
                window.addEventListener('load', function() {
                    GraphQLPlayground.init(document.getElementById('root'), {
                        endpoint: '/graphql'
                    })
                })
            </script>
        </body>
        </html>
    "#,
    )
}
