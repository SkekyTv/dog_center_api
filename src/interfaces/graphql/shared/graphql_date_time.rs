use async_graphql::{InputValueError, InputValueResult, Scalar, ScalarType, Value};
use chrono::{DateTime, Utc};
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
pub struct GraphQLDateTime(pub DateTime<Utc>);

#[Scalar(name = "DateTime")]
impl ScalarType for GraphQLDateTime {
    fn parse(value: Value) -> InputValueResult<Self> {
        if let Value::String(date_str) = &value {
            DateTime::from_str(date_str)
                .map(GraphQLDateTime)
                .map_err(|_| InputValueError::custom("Invalid DateTime format"))
        } else {
            Err(InputValueError::expected_type(value))
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.0.to_rfc3339())
    }
}
