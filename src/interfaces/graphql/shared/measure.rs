use async_graphql::{Enum, SimpleObject};

#[derive(Enum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum Unit {
    G,
}

#[derive(SimpleObject, Debug, PartialEq)]
pub struct Measure {
    pub value: i32,
    pub unit: Unit,
}
