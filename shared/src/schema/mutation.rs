use super::{models::auth::Tokens, SeaGraphContext};
use crate::database::repositories::user::UserRepository;
use crate::error::AppError;
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

impl Mutation {
  async fn impl_sign_up(
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

  async fn impl_sign_in(
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
