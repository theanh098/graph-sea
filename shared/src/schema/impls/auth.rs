use async_graphql::Context;

use crate::{
  database::repositories::user::UserRepository,
  error::AppError,
  schema::{models::auth::Tokens, mutation::Mutation, SeaGraphContext},
};

impl Mutation {
  pub async fn impl_sign_up(
    ctx: &Context<'_>,
    name: String,
    password: String,
  ) -> Result<Tokens, AppError> {
    let conn = ctx.get_database_connection().await?;

    UserRepository::new(conn)
      .create(name.as_str(), password.as_str())
      .await
      .map(|user| async {
        let mut redis_connection = ctx.get_redis_connection().await?;
        Tokens::from_entity(user, &mut redis_connection).await
      })?
      .await
  }

  pub async fn impl_sign_in(
    ctx: &Context<'_>,
    name: String,
    password: String,
  ) -> Result<Tokens, AppError> {
    let conn = ctx.get_database_connection().await?;
    let user = UserRepository::new(conn).get_user_by_name(name).await?;

    bcrypt::verify(password, &user.password)
      .map_err(|err| AppError::ExecutionError(err.to_string()))?
      .then(|| user)
      .ok_or(AppError::UnAuthorized("Invalid password"))
      .map(|user| async {
        let mut redis_connection = ctx.get_redis_connection().await?;
        Tokens::from_entity(user, &mut redis_connection).await
      })?
      .await
  }
}
