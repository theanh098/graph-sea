use async_graphql::{Object, Result};

use super::models::user::UserModel;

pub struct Query;

#[Object]
impl Query {
  async fn user(&self, name: String) -> Result<Option<UserModel>> {
    Ok(Some(UserModel { id: 1, name }))
  }
}
