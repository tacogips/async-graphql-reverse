use async_graphql::*;
#[derive(Debug, Clone)]
pub struct Url(pub String);
#[Scalar]
impl ScalarType for Url {
    fn parse(value: Value) -> InputValueResult<Self> {
        match value {
            Value::String(s) => Ok(Url(s)),
            _ => Err(InputValueError::expected_type(value)),
        }
    }
    fn to_value(&self) -> Value {
        Value::String(self.0.to_string())
    }
}
