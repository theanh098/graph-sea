use axum::{
  async_trait,
  extract::FromRequestParts,
  headers::{authorization::Bearer, Authorization},
  http::request::Parts,
  RequestPartsExt, TypedHeader,
};
use jsonwebtoken::{DecodingKey, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
  pub id: i32,
  pub exp: u32,
  pub name: String,
}

pub struct OptionalGuard(pub Option<Claims>);

impl OptionalGuard {
  pub fn into_inner(&self) -> Option<Claims> {
    self.0.clone()
  }
}

#[async_trait]
impl<S> FromRequestParts<S> for OptionalGuard
where
  S: Send + Sync,
{
  type Rejection = ();

  async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
    let access_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set.");

    parts
      .extract::<TypedHeader<Authorization<Bearer>>>()
      .await
      .map_or(Ok(OptionalGuard(None)), |bearer| {
        jsonwebtoken::decode::<Claims>(
          bearer.token(),
          &DecodingKey::from_secret(access_secret.as_bytes()),
          &Validation::default(),
        )
        .map_or(Ok(OptionalGuard(None)), |token_data| {
          Ok(OptionalGuard(Some(token_data.claims)))
        })
      })
  }
}
