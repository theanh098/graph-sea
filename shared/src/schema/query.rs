use super::models::{paginate::Paginate, post::PostModel, user::UserModel};
use async_graphql::{Context, ErrorExtensions, InputObject, Object, Result};
pub struct Query;

#[derive(InputObject)]
struct Coordinate {
  take: i32,
  cursor: i32,
}

#[Object]
impl Query {
  #[graphql(name = "me")]
  async fn get_me<'ctx>(&self, ctx: &Context<'ctx>) -> Result<UserModel> {
    Self::impl_get_me(ctx).await.map_err(|err| err.extend())
  }

  async fn get_posts<'ctx>(
    &self,
    ctx: &Context<'ctx>,
    option: Coordinate,
  ) -> Result<Paginate<PostModel>> {
    Self::impl_get_posts(ctx, option.take, option.cursor)
      .await
      .map_err(|err| err.extend())
  }
}
