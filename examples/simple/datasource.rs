use super::output::*;

pub struct DataSource;

impl DataSource {
    pub async fn create_friend_mutation_payload_friend(
        &self,
        _paylod: &CreateFriendMutationPayload,
    ) -> Friend {
        Friend {
            id: "test_id".into(),
        }
    }

    pub async fn mutation_create_friend_mutation(
        &self,
        mutation: &Mutation,
        input: CreateFriendMutationInput,
    ) -> Option<CreateFriendMutationPayload> {
        None
    }

    pub async fn friend_name(&self, frinend: &Friend) -> String {
        "john".to_string()
    }

    pub async fn friend_connection_nodes(
        &self,
        connection: &FriendConnection,
    ) -> Vec<Option<Friend>> {
        vec![]
    }

    pub async fn me_friends(&self, me: &Me, first: Option<i64>) -> FriendConnection {
        FriendConnection { total_count: 10 }
    }

    pub async fn me_notifications(&self, me: &Me) -> Vec<Option<Notification>> {
        vec![]
    }

    pub async fn me_web(&self, me: &Me) -> Option<Url> {
        None
    }

    pub async fn me_search(&self, me: &Me, text: String) -> Vec<Option<SearchResult>> {
        vec![]
    }

    pub async fn me_status(&self, me: &Me) -> Option<Status> {
        None
    }

    pub async fn query_me(&self, q: &Query) -> Me {
        Me {
            id: "1".into(),
            name: "smith".into(),
            rank: 12.2,
            email: None,
            age: None,
            active: None,
        }
    }

    pub async fn query_active(&self, q: &Query) -> bool {
        true
    }

    pub async fn query_type(&self, q: &Query) -> Option<String> {
        None
    }

    pub async fn notification_friends(
        &self,
        notification: &Notification,
        first: Option<i64>,
        num: Option<i64>,
    ) -> FriendConnection {
        FriendConnection { total_count: 20 }
    }
}
