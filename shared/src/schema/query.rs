use async_graphql::{Context, Object, Result};

use crate::security::authentication::Claims;

use super::models::user::UserModel;

pub struct Query;

#[Object]
impl Query {
  async fn user(&self, ctx: &Context<'_>, name: String) -> Result<Option<UserModel>> {
    let claims = ctx.data::<Claims>();

    dbg!(claims);

    Ok(Some(UserModel {
      id: 1,
      name,
      password: "cmm".into(),
    }))
  }
}
