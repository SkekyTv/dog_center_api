use async_graphql::{InputValueError, InputValueResult, Scalar, ScalarType, Value};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct GraphQLUuid(pub Uuid);

#[Scalar(name = "UUID")]
impl ScalarType for GraphQLUuid {
    fn parse(value: Value) -> InputValueResult<Self> {
        if let Value::String(uuid_str) = &value {
            Uuid::parse_str(uuid_str)
                .map(GraphQLUuid)
                .map_err(|_| InputValueError::custom("Invalid UUID format"))
        } else {
            Err(InputValueError::expected_type(value))
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.0.to_string())
    }
}
