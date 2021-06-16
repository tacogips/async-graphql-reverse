use async_graphql::*;
#[derive(Debug, InputObject)]
pub struct CreateFriendMutationInput {
    pub user_id: ID,
}
