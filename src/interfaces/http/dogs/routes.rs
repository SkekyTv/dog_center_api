use axum::{
    Router,
    routing::{get, post},
};

use crate::app_state::AppState;

use super::handlers::{create_dog_handler, get_dog_handler};

pub fn dogs_routes() -> Router<AppState> {
    Router::new()
        .route("/dogs/{id}", get(get_dog_handler))
        .route("/dogs", post(create_dog_handler))
}
