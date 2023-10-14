use crate::error::AppError;
use crate::{database::entities::user::Model as UserEntity, schema::RedisConnection};
use axum::{
  async_trait,
  extract::FromRequestParts,
  headers::{authorization::Bearer, Authorization},
  http::request::Parts,
  RequestPartsExt, TypedHeader,
};
use chrono::Utc;
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
  pub id: i32,
  pub name: String,
  pub exp: u32,
}

impl Claims {
  pub fn new(id: i32, name: String, expired: chrono::Duration) -> Self {
    Self {
      id,
      name,
      exp: Utc::now().checked_add_signed(expired).unwrap().timestamp() as u32,
    }
  }
}

pub struct Guard(pub Option<Claims>, pub &'static str);

impl Guard {
  pub fn retrieve_claims(&self) -> Option<Claims> {
    self.0.clone()
  }

  pub fn retrieve_err_msg(&self) -> &'static str {
    self.1
  }
}

#[async_trait]
impl<S> FromRequestParts<S> for Guard
where
  S: Send + Sync,
{
  type Rejection = ();

  async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
    let access_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set.");

    parts
      .extract::<TypedHeader<Authorization<Bearer>>>()
      .await
      .map_or(Ok(Guard(None, "Missing Bearer token")), |bearer| {
        jsonwebtoken::decode::<Claims>(
          bearer.token(),
          &DecodingKey::from_secret(access_secret.as_bytes()),
          &Validation::default(),
        )
        .map_or_else(
          |err| match err.kind() {
            ErrorKind::ExpiredSignature => Ok(Guard(None, "Expired token")),
            _ => Ok(Guard(None, "Invalid token")),
          },
          |token_data| Ok(Guard(Some(token_data.claims), "")),
        )
      })
  }
}

pub async fn generate_tokens(
  user: UserEntity,
  redis_connection: &mut RedisConnection,
) -> Result<(String, String), AppError> {
  let access_token_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
  let refresh_token_secret =
    std::env::var("JWT_REFRESH_SECRET").expect("JWT_REFRESH_SECRET must be set");

  let access_token = encode(
    &Header::default(),
    &Claims::new(user.id, user.name.clone(), chrono::Duration::days(3)),
    &EncodingKey::from_secret(access_token_secret.as_bytes()),
  )
  .map_err(|err| AppError::ExecutionError(err.to_string()))?;

  let refresh_token = encode(
    &Header::default(),
    &Claims::new(user.id, user.name, chrono::Duration::days(180)),
    &EncodingKey::from_secret(refresh_token_secret.as_bytes()),
  )
  .map_err(|err| AppError::ExecutionError(err.to_string()))?;

  deadpool_redis::redis::cmd("SET")
    .arg(format!("refresh_token_on_user_{}", user.id).as_str())
    .arg(&refresh_token)
    .query_async(redis_connection)
    .await
    .map_err(|err| AppError::ExecutionError(err.to_string()))?;

  Ok((access_token, refresh_token))
}
