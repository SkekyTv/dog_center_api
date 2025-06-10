use std::sync::Arc;

use async_graphql::{Context, Error, InputObject};
use base64::{Engine, prelude::BASE64_STANDARD};

use crate::{
    infra::db::dogs_repository::PgDogsRepository,
    interfaces::graphql::{
        dogs::dogs_types::DogEdge,
        shared::cursor_pagination::{CursorPagination, PageInfo},
    },
    repositories::dogs_repository::ListDogInput,
    use_cases::dogs_service::DogsService,
};

use super::{dog_mapper, dogs_types::DogConnection};

#[derive(InputObject)]
pub struct DogsInput {
    pub cursor_pagination: CursorPagination,
}

pub async fn dogs(ctx: &Context<'_>, input: DogsInput) -> Result<DogConnection, Error> {
    let dogs_service = ctx
        .data::<Arc<DogsService<PgDogsRepository>>>()
        .map_err(|_| Error::new("DogService not found in context"))?;

    let after_id = input
        .cursor_pagination
        .after
        .as_ref()
        .map(|cursor| String::from_utf8(BASE64_STANDARD.decode(cursor).unwrap()).unwrap());

    let dogs = dogs_service
        .list_dogs(ListDogInput {
            after_id,
            first: input.cursor_pagination.first,
        })
        .await
        .map_err(|e| Error::new(format!("Error listing dog: {}", e)))?;

    Ok(DogConnection {
        edges: dogs
            .edges
            .into_iter()
            .map(|e| DogEdge {
                node: dog_mapper::map_dog_to_gql(e.node),
                cursor: e.cursor,
            })
            .collect(),
        page_info: PageInfo {
            end_cursor: dogs.page_info.end_cursor,
            has_next_page: dogs.page_info.has_next_page,
        },
    })
}
