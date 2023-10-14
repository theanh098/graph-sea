use async_graphql::{Context, ErrorExtensions, Object, Result};

use crate::{database::repositories::user::UserRepository, error::AppError};

use super::{models::user::UserModel, SeaGraphContext};

pub struct Query;

#[Object]
impl Query {
  #[graphql(name = "me")]
  async fn get_me<'ctx>(&self, ctx: &Context<'ctx>) -> Result<UserModel> {
    Self::impl_get_me(ctx).await.map_err(|err| err.extend())
  }
}

impl Query {
  async fn impl_get_me<'ctx>(ctx: &Context<'ctx>) -> Result<UserModel, AppError> {
    let claims = ctx.get_claims().await?;
    ctx
      .get_database_connection()
      .await
      .map(|conn| async move {
        UserRepository::new(conn)
          .get_user_by_id(claims.id)
          .await
          .map(|user| UserModel::from(user))
      })?
      .await
  }
}
