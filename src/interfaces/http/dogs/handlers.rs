use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;

use crate::{app_state::AppState, entities::dogs::Dogs, shared::types::sex::Sex};

use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateDogRequest {
    #[validate(length(min = 1))]
    pub name: String,
    pub birthdate: Option<DateTime<Utc>>,

    #[serde(default)] // default []
    pub races: Vec<String>,

    pub sex: Sex,
}

pub async fn get_dog_handler(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<Dogs>, (StatusCode, String)> {
    let dogs_service = &state.dogs_service;

    let dog = dogs_service
        .get_dog(id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    match dog {
        Some(dog) => Ok(Json(Dogs {
            id: dog.id,
            name: dog.name,
            birthdate: dog.birthdate,
            races: dog.races,
            img_url: dog.img_url,
            sex: dog.sex,
        })),
        None => Err((StatusCode::NOT_FOUND, format!("Dog with id {id} not found"))),
    }
}

pub async fn create_dog_handler(
    State(state): State<AppState>,
    Json(payload): Json<CreateDogRequest>,
) -> Result<(StatusCode, Json<Dogs>), (StatusCode, String)> {
    let dog = Dogs::new(payload.name, payload.sex, payload.birthdate, payload.races);

    let repo = &state.dogs_service;

    match repo.create_dog(dog.clone()).await {
        Ok(_) => Ok((StatusCode::CREATED, Json(dog))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}
