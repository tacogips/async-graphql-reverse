use super::objects::Friend;
use super::objects::Notification;
use async_graphql::*;
#[derive(Union)]
pub enum SearchResult {
    Friend(Friend),
    Notification(Notification),
}
