use super::output::*;
use async_graphql::*;
pub struct DataSource {}
impl DataSource {
    pub async fn query_me(&self, ctx: &Context<'_>, object: &Query) -> Result<Me> {
        unimplemented!("resolver {} is unimpemented yet", "query_me")
    }
    pub async fn query_active(&self, ctx: &Context<'_>, object: &Query) -> Result<bool> {
        unimplemented!("resolver {} is unimpemented yet", "query_active")
    }
    pub async fn query_type(&self, ctx: &Context<'_>, object: &Query) -> Result<Option<String>> {
        unimplemented!("resolver {} is unimpemented yet", "query_type")
    }
    pub async fn mutation_create_friend_mutation(
        &self,
        ctx: &Context<'_>,
        object: &Mutation,
        input: CreateFriendMutationInput,
    ) -> Result<Option<CreateFriendMutationPayload>> {
        unimplemented!(
            "resolver {} is unimpemented yet",
            "mutation_create_friend_mutation"
        )
    }
    pub async fn create_friend_mutation_payload_friend(
        &self,
        ctx: &Context<'_>,
        object: &CreateFriendMutationPayload,
    ) -> Result<Friend> {
        unimplemented!(
            "resolver {} is unimpemented yet",
            "create_friend_mutation_payload_friend"
        )
    }
    pub async fn friend_name(&self, ctx: &Context<'_>, object: &Friend) -> Result<String> {
        unimplemented!("resolver {} is unimpemented yet", "friend_name")
    }
    pub async fn friend_connection_nodes(
        &self,
        ctx: &Context<'_>,
        object: &FriendConnection,
    ) -> Result<Vec<Option<Friend>>> {
        unimplemented!("resolver {} is unimpemented yet", "friend_connection_nodes")
    }
    pub async fn me_friends(
        &self,
        ctx: &Context<'_>,
        object: &Me,
        first: Option<i64>,
        limit: Option<i64>,
        sort_direction: Option<SortDirection>,
        next_token: Option<String>,
    ) -> Result<FriendConnection> {
        unimplemented!("resolver {} is unimpemented yet", "me_friends")
    }
    pub async fn me_notifications(
        &self,
        ctx: &Context<'_>,
        object: &Me,
    ) -> Result<Vec<Option<Notification>>> {
        unimplemented!("resolver {} is unimpemented yet", "me_notifications")
    }
    pub async fn me_search(
        &self,
        ctx: &Context<'_>,
        object: &Me,
        text: String,
    ) -> Result<Vec<Option<SearchResult>>> {
        unimplemented!("resolver {} is unimpemented yet", "me_search")
    }
    pub async fn me_status(&self, ctx: &Context<'_>, object: &Me) -> Result<Option<Status>> {
        unimplemented!("resolver {} is unimpemented yet", "me_status")
    }
    pub async fn notification_friends(
        &self,
        ctx: &Context<'_>,
        object: &Notification,
        first: Option<i64>,
        num: Option<i64>,
    ) -> Result<FriendConnection> {
        unimplemented!("resolver {} is unimpemented yet", "notification_friends")
    }

    pub async fn query_custom_resolver(
        &self,
        ctx: &Context<'_>,
        object: &Query,
    ) -> Result<Option<String>> {
        unimplemented!()
    }
}
