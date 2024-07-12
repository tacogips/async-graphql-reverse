// DO NOT EDIT THIS FILE
// This file was generated by https://github.com/tacogips/async-graphql-reverse
use async_graphql::*;
#[derive(Enum, Copy, Clone, Debug, Eq, PartialEq)]
#[graphql(rename_items = "camelCase")]
pub enum Status {
    Registered,
    EmailVerified,
}
#[derive(Enum, Copy, Clone, Debug, Eq, PartialEq)]
#[graphql(rename_items = "snake_case")]
pub enum Sex {
    Male,
    Female,
    Other,
}
#[derive(Enum, Copy, Clone, Debug, Eq, PartialEq)]
#[graphql(rename_items = "UPPERCASE")]
pub enum UserType {
    NotPayed,
    Payed,
    #[graphql(name = "OTHER")]
    Other,
}
#[derive(Enum, Copy, Clone, Debug, Eq, PartialEq)]
#[graphql(rename_items = "camelCase")]
pub enum SortDirection {
    Asc,
    Desc,
}
