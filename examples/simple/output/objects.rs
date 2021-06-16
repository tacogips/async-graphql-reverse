use super::input_objects::CreateFriendMutationInput;
use super::scalars::Url;
use super::unions::SearchResult;
use crate::datasource::DataSource;
use async_graphql::*;
#[derive(Debug)]
pub struct Query {}
#[Object]
impl Query {
    #[doc = "me: Single-line comment"]
    pub async fn me(&self, ctx: &Context<'_>) -> Option<Vec<SearchResult>> {
        ctx.data_unchecked::<DataSource>().query_me(&self)
    }
    pub async fn active(&self, ctx: &Context<'_>) -> Option<Vec<SearchResult>> {
        ctx.data_unchecked::<DataSource>().query_active(&self)
    }
}
#[derive(Debug)]
pub struct Mutation {}
#[Object]
impl Mutation {
    pub async fn create_friend_mutation(
        &self,
        ctx: &Context<'_>,
        input: CreateFriendMutationInput,
    ) -> Option<Vec<SearchResult>> {
        ctx.data_unchecked::<DataSource>()
            .mutation_create_friend_mutation(&self, input)
    }
}
#[derive(Debug)]
pub struct Subscription {
    pub badge: i64,
}
#[Object]
impl Subscription {
    pub async fn badge(&self) -> i64 {
        self.badge.clone()
    }
}
#[derive(Debug)]
pub struct CreateFriendMutationPayload {}
#[Object]
impl CreateFriendMutationPayload {
    pub async fn friend(&self, ctx: &Context<'_>) -> Option<Vec<SearchResult>> {
        ctx.data_unchecked::<DataSource>()
            .create_friend_mutation_payload_friend(&self)
    }
}
#[derive(Debug)]
pub struct Friend {
    pub id: ID,
    pub name: String,
}
#[Object]
impl Friend {
    pub async fn id(&self) -> ID {
        self.id.clone()
    }
    pub async fn name(&self) -> String {
        self.name.clone()
    }
}
#[derive(Debug)]
pub struct FriendConnection {
    pub total_count: i64,
}
#[Object]
impl FriendConnection {
    pub async fn total_count(&self) -> i64 {
        self.total_count.clone()
    }
    pub async fn nodes(&self, ctx: &Context<'_>) -> Option<Vec<SearchResult>> {
        ctx.data_unchecked::<DataSource>()
            .friend_connection_nodes(&self)
    }
}
#[derive(Debug)]
pub struct Me {
    pub id: ID,
    pub name: String,
    pub rank: f64,
    pub email: Option<String>,
    pub age: Option<i64>,
    pub active: Option<bool>,
}
#[Object]
impl Me {
    pub async fn id(&self) -> ID {
        self.id.clone()
    }
    pub async fn name(&self) -> String {
        self.name.clone()
    }
    pub async fn rank(&self) -> f64 {
        self.rank.clone()
    }
    pub async fn email(&self) -> Option<String> {
        self.email.clone()
    }
    pub async fn age(&self) -> Option<i64> {
        self.age.clone()
    }
    pub async fn active(&self) -> Option<bool> {
        self.active.clone()
    }
    pub async fn friends(
        &self,
        ctx: &Context<'_>,
        first: Option<i64>,
    ) -> Option<Vec<SearchResult>> {
        ctx.data_unchecked::<DataSource>().me_friends(&self, first)
    }
    pub async fn notifications(&self, ctx: &Context<'_>) -> Option<Vec<SearchResult>> {
        ctx.data_unchecked::<DataSource>().me_notifications(&self)
    }
    pub async fn web(&self, ctx: &Context<'_>) -> Option<Vec<SearchResult>> {
        ctx.data_unchecked::<DataSource>().me_web(&self)
    }
    pub async fn search(&self, ctx: &Context<'_>, text: String) -> Option<Vec<SearchResult>> {
        ctx.data_unchecked::<DataSource>().me_search(&self, text)
    }
}
#[derive(Debug)]
pub struct Notification {
    pub id: ID,
    pub title: String,
}
#[Object]
impl Notification {
    pub async fn id(&self) -> ID {
        self.id.clone()
    }
    pub async fn title(&self) -> String {
        self.title.clone()
    }
    pub async fn friends(
        &self,
        ctx: &Context<'_>,
        first: Option<i64>,
        num: Option<i64>,
    ) -> Option<Vec<SearchResult>> {
        ctx.data_unchecked::<DataSource>()
            .notification_friends(&self, first, num)
    }
}