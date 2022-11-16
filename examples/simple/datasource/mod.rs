use super::output::*;
use async_graphql::*;

pub struct DataSource;
impl DataSource {
    pub async fn query_me(&self, _ctx: &Context<'_>, _object: &Query) -> Result<Me> {
        unimplemented!("resolver {} is unimpemented yet", "query_me")
    }
    pub async fn query_active(&self, _ctx: &Context<'_>, _object: &Query) -> Result<bool> {
        unimplemented!("resolver {} is unimpemented yet", "query_active")
    }
    pub async fn query_type(&self, _ctx: &Context<'_>, _object: &Query) -> Result<Option<String>> {
        unimplemented!("resolver {} is unimpemented yet", "query_type")
    }
    pub async fn mutation_create_friend_mutation(
        &self,
        _ctx: &Context<'_>,
        _object: &Mutation,
        _input: CreateFriendMutationInput,
    ) -> Result<Option<CreateFriendMutationPayload>> {
        unimplemented!(
            "resolver {} is unimpemented yet",
            "mutation_create_friend_mutation"
        )
    }
    pub async fn create_friend_mutation_payload_friend(
        &self,
        _ctx: &Context<'_>,
        _object: &CreateFriendMutationPayload,
    ) -> Result<Friend> {
        unimplemented!(
            "resolver {} is unimpemented yet",
            "create_friend_mutation_payload_friend"
        )
    }
    pub async fn me_user_type(&self, _ctx: &Context<'_>, _object: &Me) -> Result<Option<UserType>> {
        unimplemented!("resolver {} is unimpemented yet", "me_user_type")
    }
    pub async fn friend_name(&self, _ctx: &Context<'_>, _object: &Friend) -> Result<String> {
        unimplemented!("resolver {} is unimpemented yet", "friend_name")
    }
    pub async fn friend_sex(&self, _ctx: &Context<'_>, _object: &Friend) -> Result<Sex> {
        unimplemented!("resolver {} is unimpemented yet", "friend_sex")
    }
    pub async fn friend_user_type(
        &self,
        _ctx: &Context<'_>,
        _object: &Friend,
    ) -> Result<Option<UserType>> {
        unimplemented!("resolver {} is unimpemented yet", "friend_user_type")
    }
    pub async fn friend_others(
        &self,
        _ctx: &Context<'_>,
        _object: &Friend,
    ) -> Result<Option<Vec<Option<Friend>>>> {
        unimplemented!("resolver {} is unimpemented yet", "friend_others")
    }
    pub async fn friend_connection_nodes(
        &self,
        _ctx: &Context<'_>,
        _object: &FriendConnection,
    ) -> Result<Vec<Option<Friend>>> {
        unimplemented!("resolver {} is unimpemented yet", "friend_connection_nodes")
    }
    pub async fn notification_friends(
        &self,
        _ctx: &Context<'_>,
        _object: &Notification,
        _first: Option<i64>,
        _num: Option<i64>,
    ) -> Result<FriendConnection> {
        unimplemented!("resolver {} is unimpemented yet", "notification_friends")
    }

    pub async fn query_custom_resolver(
        &self,
        _ctx: &Context<'_>,
        _object: &Query,
    ) -> Result<Option<String>> {
        unimplemented!()
    }
}
