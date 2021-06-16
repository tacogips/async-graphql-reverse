mod objects;
use objects::*;
mod input_objects;
use input_objects::*;
mod unions;
use unions::*;
mod scalars;
use scalars::*;
mod interfaces;
use async_graphql::*;
use interfaces::*;
pub fn schema() -> Schema<Query, Mutation, EmptySubscription> {
    Schema::new(Query {}, Mutation {}, EmptySubscription)
}
