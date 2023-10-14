use crate::{
  database::entities::user::Model as UserEntity, error::AppError, schema::RedisConnection,
  security::authentication::generate_tokens,
};
use async_graphql::SimpleObject;

#[derive(SimpleObject)]
pub struct Tokens {
  access_token: String,
  refresh_token: String,
}

impl Tokens {
  pub async fn from_entity(
    user: UserEntity,
    redis_connection: &mut RedisConnection,
  ) -> Result<Self, AppError> {
    let (access_token, refresh_token) = generate_tokens(user, redis_connection).await?;

    Ok(Self {
      access_token,
      refresh_token,
    })
  }
}
