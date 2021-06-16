use super::objects::Friend;
use super::objects::Me;
use async_graphql::*;
#[derive(Interface)]
#[graphql(field(name = "id", type = "ID"), field(name = "name", type = "String"))]
#[derive(Debug)]
pub enum User {
    Me(Me),
    Friend(Friend),
}
