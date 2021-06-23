mod objects;
use objects::*;
mod input_objects;
use input_objects::*;
mod unions;
use unions::*;
mod scalars;
use scalars::*;
mod interfaces;
use interfaces::*;
mod enums;
use async_graphql::*;
use enums::*;
pub fn schema() -> Schema<Query, Mutation, EmptySubscription> {
    Schema::new(Query {}, Mutation {}, EmptySubscription)
}
