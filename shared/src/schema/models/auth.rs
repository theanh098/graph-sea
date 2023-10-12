use crate::{
  database::entities::user::Model as UserEntity, error::SeaGraphError, schema::RedisConnection,
  security::authentication::generate_tokens,
};
use async_graphql::SimpleObject;

#[derive(SimpleObject)]
#[graphql(rename_fields = "camelCase")]
pub struct AuthResponse {
  access_token: String,
  refresh_token: String,
}

impl AuthResponse {
  pub async fn from_entity(
    user: UserEntity,
    redis_connection: &mut RedisConnection,
  ) -> Result<Self, SeaGraphError> {
    let (access_token, refresh_token) = generate_tokens(user, redis_connection).await?;

    Ok(Self {
      access_token,
      refresh_token,
    })
  }
}
