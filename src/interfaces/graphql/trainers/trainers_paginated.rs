use std::sync::Arc;

use async_graphql::{Context, Error, InputObject};
use base64::{Engine, prelude::BASE64_STANDARD};

use crate::{
    infra::db::trainer_repository::PgTrainersRepository,
    interfaces::graphql::shared::cursor_pagination::{CursorPagination, PageInfo},
    repositories::trainers_repository::ListTrainerInput,
    use_cases::trainers_service::TrainersService,
};

use super::{
    trainer_mapper,
    trainers_types::{TrainerConnection, TrainerEdge},
};

#[derive(InputObject)]
pub struct TrainersInput {
    pub cursor_pagination: CursorPagination,
}

pub async fn trainers(ctx: &Context<'_>, input: TrainersInput) -> Result<TrainerConnection, Error> {
    let trainers_service = ctx
        .data::<Arc<TrainersService<PgTrainersRepository>>>()
        .map_err(|_| Error::new("TrainersService not found in context"))?;

    let after_id = match input.cursor_pagination.after.as_ref() {
        Some(cursor) => {
            let decoded = BASE64_STANDARD
                .decode(cursor)
                .map_err(|_| Error::new("Malformed cursor: base64 decode failed"))?;
            let as_str = String::from_utf8(decoded)
                .map_err(|_| Error::new("Malformed cursor: invalid UTF-8"))?;
            Some(as_str)
        }
        None => None,
    };

    let trainers = trainers_service
        .list_trainers(ListTrainerInput {
            after_id,
            first: input.cursor_pagination.first,
        })
        .await
        .map_err(|e| Error::new(format!("Error listing trainers: {}", e)))?;

    Ok(TrainerConnection {
        edges: trainers
            .edges
            .into_iter()
            .map(|e| TrainerEdge {
                node: trainer_mapper::map_trainer_to_gql(e.node),
                cursor: e.cursor,
            })
            .collect(),
        page_info: PageInfo {
            end_cursor: trainers.page_info.end_cursor,
            has_next_page: trainers.page_info.has_next_page,
        },
    })
}
