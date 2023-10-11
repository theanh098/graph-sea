pub mod models;

pub use async_graphql::http::GraphiQLSource;
use async_graphql::{EmptySubscription, Schema};
use async_graphql::{Object, Result};
use models::user::User;

pub struct Query;
pub struct Mutation;

#[Object]
impl Query {
  async fn user(&self, name: String) -> Result<Option<User>> {
    Ok(Some(User { id: 1, name }))
  }
}

#[Object]
impl Mutation {
  async fn signup(&self, _username: String, _password: String) -> Result<bool> {
    Ok(true)
  }
}

pub fn init_schema() -> Schema<Query, Mutation, EmptySubscription> {
  Schema::build(Query, Mutation, EmptySubscription).finish()
}
