use super::output::*;

pub struct DataSource;
use async_graphql::*;

impl DataSource {
    pub async fn create_friend_mutation_payload_friend(
        &self,
        _ctx: &Context<'_>,
        _paylod: &CreateFriendMutationPayload,
    ) -> Friend {
        Friend {
            id: "test_id".into(),
        }
    }

    pub async fn mutation_create_friend_mutation(
        &self,
        _ctx: &Context<'_>,
        _mutation: &Mutation,
        _input: CreateFriendMutationInput,
    ) -> Option<CreateFriendMutationPayload> {
        None
    }

    pub async fn friend_name(&self, _ctx: &Context<'_>, _frinend: &Friend) -> String {
        "john".to_string()
    }

    pub async fn friend_connection_nodes(
        &self,
        _ctx: &Context<'_>,
        _connection: &FriendConnection,
    ) -> Vec<Option<Friend>> {
        vec![]
    }

    pub async fn me_friends(
        &self,
        _ctx: &Context<'_>,
        _me: &Me,
        _first: Option<i64>,
    ) -> FriendConnection {
        FriendConnection { total_count: 10 }
    }

    pub async fn me_notifications(
        &self,
        _ctx: &Context<'_>,
        _me: &Me,
    ) -> Vec<Option<Notification>> {
        vec![]
    }

    pub async fn me_web(&self, _ctx: &Context<'_>, _me: &Me) -> Option<Url> {
        None
    }

    pub async fn me_search(
        &self,
        _ctx: &Context<'_>,
        _me: &Me,
        _text: String,
    ) -> Vec<Option<SearchResult>> {
        vec![]
    }

    pub async fn me_status(&self, _ctx: &Context<'_>, _me: &Me) -> Option<Status> {
        None
    }

    pub async fn query_me(&self, _ctx: &Context<'_>, _q: &Query) -> Me {
        Me {
            id: "1".into(),
            name: "smith".into(),
            rank: 12.2,
            email: None,
            age: None,
            active: None,
        }
    }

    pub async fn query_active(&self, _ctx: &Context<'_>, _q: &Query) -> bool {
        true
    }

    pub async fn query_type(&self, _ctx: &Context<'_>, _q: &Query) -> Option<String> {
        None
    }

    pub async fn notification_friends(
        &self,
        _ctx: &Context<'_>,
        _notification: &Notification,
        _first: Option<i64>,
        _num: Option<i64>,
    ) -> FriendConnection {
        FriendConnection { total_count: 20 }
    }
}
