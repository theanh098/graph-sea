use super::models::auth::Tokens;
use async_graphql::{Context, ErrorExtensions, Object, Result};

pub struct Mutation;

#[Object]
impl Mutation {
  async fn signup(&self, ctx: &Context<'_>, name: String, password: String) -> Result<Tokens> {
    Self::impl_sign_up(ctx, name, password)
      .await
      .map_err(|err| err.extend())
  }

  async fn sign_in(&self, ctx: &Context<'_>, name: String, password: String) -> Result<Tokens> {
    Self::impl_sign_in(ctx, name, password)
      .await
      .map_err(|err| err.extend())
  }
}
