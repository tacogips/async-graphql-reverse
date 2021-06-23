use async_graphql::*;
#[derive(Enum, Copy, Clone, Debug, Eq, PartialEq)]
pub enum Status {
    Registered,
    EmailVerified,
}
