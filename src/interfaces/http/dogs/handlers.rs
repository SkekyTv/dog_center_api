use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{app_state::AppState, entities::dogs::Dogs};

#[derive(Serialize)]
pub struct DogsResponse {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateDogRequest {
    pub name: String,
}

pub async fn get_dog_handler(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<DogsResponse>, (StatusCode, String)> {
    let dogs_service = &state.dogs_service;

    let dog = dogs_service
        .get_dog(id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    match dog {
        Some(dog) => Ok(Json(DogsResponse {
            id: dog.id,
            name: dog.name,
        })),
        None => Err((StatusCode::NOT_FOUND, format!("Dog with id {id} not found"))),
    }
}

pub async fn create_dog_handler(
    State(state): State<AppState>,
    Json(payload): Json<CreateDogRequest>,
) -> Result<(StatusCode, Json<Dogs>), (StatusCode, String)> {
    let dog = Dogs {
        id: Uuid::new_v4(),
        name: payload.name,
    };

    let repo = &state.dogs_service;

    match repo.create_dog(dog.clone()).await {
        Ok(_) => Ok((StatusCode::CREATED, Json(dog))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}
