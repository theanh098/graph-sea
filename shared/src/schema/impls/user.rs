use async_graphql::Context;

use crate::{
  database::repositories::user::UserRepository,
  error::AppError,
  schema::{models::user::UserModel, query::Query, SeaGraphContext},
};

impl Query {
  pub async fn impl_get_me<'ctx>(ctx: &Context<'ctx>) -> Result<UserModel, AppError> {
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
