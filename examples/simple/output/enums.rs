// DO NOT EDIT THIS FILE
// This file was generated by https://github.com/tacogips/async-graphql-reverse
use async_graphql::*;
#[derive(Enum, Copy, Clone, Debug, Eq, PartialEq)]
pub enum Status {
    Registered,
    EmailVerified,
}
#[derive(Enum, Copy, Clone, Debug, Eq, PartialEq)]
pub enum Sex {
    Male,
    Female,
    Other,
}
#[derive(Enum, Copy, Clone, Debug, Eq, PartialEq)]
pub enum UserType {
    NotPayed,
    Payed,
}
#[derive(Enum, Copy, Clone, Debug, Eq, PartialEq)]
pub enum SortDirection {
    Asc,
    Desc,
}
