use super::{models::auth::Tokens, SeaGraphContext};
use crate::database::repositories::user::UserRepository;
use crate::error::AppError;
use async_graphql::{Context, Object, Result};

pub struct Mutation;

#[Object]
impl Mutation {
  #[graphql(name = "tokens")]
  async fn signup(
    &self,
    ctx: &Context<'_>,
    name: String,
    password: String,
  ) -> Result<Tokens, AppError> {
    let conn = ctx.get_database_connection().await?;
    let mut redis_connection = ctx.get_redis_connection().await?;

    let user = UserRepository::new(conn)
      .create(name.as_str(), password.as_str())
      .await?;

    Ok(Tokens::from_entity(user, &mut redis_connection).await?)
  }

  async fn sign_in(
    &self,
    ctx: &Context<'_>,
    name: String,
    password: String,
  ) -> Result<Tokens, AppError> {
    let conn = ctx.get_database_connection().await?;
    let mut redis_connection = ctx.get_redis_connection().await?;

    return UserRepository::new(conn)
      .get_user_by_name(name)
      .await
      .and_then(|user| {
        bcrypt::verify(password, &user.password)
          .map_err(|err| AppError::ExecutionError(err.to_string()))
          .and_then(|valid| {
            valid
              .then(|| user)
              .ok_or(AppError::AuthenticationError)
              .map(|user| async { Tokens::from_entity(user, &mut redis_connection).await })
          })
      })?
      .await;
  }
}
