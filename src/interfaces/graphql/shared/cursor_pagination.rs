use async_graphql::{InputObject, SimpleObject};

#[derive(InputObject)]
pub struct CursorPagination {
    pub first: i32,
    pub after: Option<String>,
}

#[derive(SimpleObject)]
pub struct PageInfo {
    pub end_cursor: Option<String>,
    pub has_next_page: bool,
}
