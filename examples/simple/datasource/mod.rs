use super::output::*;
use async_graphql::*;
pub struct DataSource {}
impl DataSource {
    pub async fn query_me(&self, ctx: &Context<'_>, object: &Query) -> Me {
        unimplemented!("resolver {} is unimpemented yet", "query_me")
    }
    pub async fn query_active(&self, ctx: &Context<'_>, object: &Query) -> bool {
        unimplemented!("resolver {} is unimpemented yet", "query_active")
    }
    pub async fn query_type(&self, ctx: &Context<'_>, object: &Query) -> Option<String> {
        unimplemented!("resolver {} is unimpemented yet", "query_type")
    }
    pub async fn mutation_create_friend_mutation(
        &self,
        ctx: &Context<'_>,
        object: &Mutation,
        input: CreateFriendMutationInput,
    ) -> Option<CreateFriendMutationPayload> {
        unimplemented!(
            "resolver {} is unimpemented yet",
            "mutation_create_friend_mutation"
        )
    }
    pub async fn create_friend_mutation_payload_friend(
        &self,
        ctx: &Context<'_>,
        object: &CreateFriendMutationPayload,
    ) -> Friend {
        unimplemented!(
            "resolver {} is unimpemented yet",
            "create_friend_mutation_payload_friend"
        )
    }
    pub async fn friend_name(&self, ctx: &Context<'_>, object: &Friend) -> String {
        unimplemented!("resolver {} is unimpemented yet", "friend_name")
    }
    pub async fn friend_connection_nodes(
        &self,
        ctx: &Context<'_>,
        object: &FriendConnection,
    ) -> Vec<Option<Friend>> {
        unimplemented!("resolver {} is unimpemented yet", "friend_connection_nodes")
    }
    pub async fn me_friends(
        &self,
        ctx: &Context<'_>,
        object: &Me,
        first: Option<i64>,
    ) -> FriendConnection {
        unimplemented!("resolver {} is unimpemented yet", "me_friends")
    }
    pub async fn me_notifications(
        &self,
        ctx: &Context<'_>,
        object: &Me,
    ) -> Vec<Option<Notification>> {
        unimplemented!("resolver {} is unimpemented yet", "me_notifications")
    }
    pub async fn me_search(
        &self,
        ctx: &Context<'_>,
        object: &Me,
        text: String,
    ) -> Vec<Option<SearchResult>> {
        unimplemented!("resolver {} is unimpemented yet", "me_search")
    }
    pub async fn me_status(&self, ctx: &Context<'_>, object: &Me) -> Option<Status> {
        unimplemented!("resolver {} is unimpemented yet", "me_status")
    }
    pub async fn notification_friends(
        &self,
        ctx: &Context<'_>,
        object: &Notification,
        first: Option<i64>,
        num: Option<i64>,
    ) -> FriendConnection {
        unimplemented!("resolver {} is unimpemented yet", "notification_friends")
    }
}
