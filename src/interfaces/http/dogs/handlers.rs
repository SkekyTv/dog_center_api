use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    app_state::AppState, entities::dogs::Dog, shared::types::sex::Sex,
    use_cases::dogs_service::CreateDogInput,
};

use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateDogRequest {
    #[validate(length(min = 1))]
    pub name: String,
    pub birthdate: Option<DateTime<Utc>>,

    #[serde(default)] // default []
    pub races: Vec<String>,

    pub sex: Sex,

    #[serde(default)] // default []
    pub weight: Vec<i32>,

    pub icad_id: Option<String>,
}

pub async fn get_dog_handler(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<Dog>, (StatusCode, String)> {
    let dogs_service = &state.dogs_service;

    let dog = dogs_service
        .get_dog(id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    match dog {
        Some(dog) => Ok(Json(dog)),
        None => Err((StatusCode::NOT_FOUND, format!("Dog with id {id} not found"))),
    }
}

pub async fn create_dog_handler(
    State(state): State<AppState>,
    Json(payload): Json<CreateDogRequest>,
) -> Result<(StatusCode, Json<Dog>), (StatusCode, String)> {
    let repo = &state.dogs_service;

    let dog = repo
        .create_dog(CreateDogInput {
            name: payload.name,
            sex: payload.sex,
            birthdate: payload.birthdate,
            races: payload.races,
            weight: payload.weight,
            icad_id: payload.icad_id,
        })
        .await;

    match dog {
        Ok(d) => Ok((StatusCode::CREATED, Json(d))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}
