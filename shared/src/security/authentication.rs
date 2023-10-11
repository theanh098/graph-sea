use axum::{
  async_trait,
  extract::FromRequestParts,
  headers::{authorization::Bearer, Authorization},
  http::{request::Parts, StatusCode},
  response::{IntoResponse, Response},
  Json, RequestPartsExt, TypedHeader,
};
use jsonwebtoken::{errors::ErrorKind, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
  pub id: i32,
  pub exp: u32,
  pub name: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
  S: Send + Sync,
{
  type Rejection = AuthError;

  async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
    let access_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set.");

    let TypedHeader(Authorization(bearer)) = parts
      .extract::<TypedHeader<Authorization<Bearer>>>()
      .await
      .map_err(|err| AuthError::InvalidToken(err.to_string()))?;

    let token_data = jsonwebtoken::decode::<Claims>(
      bearer.token(),
      &DecodingKey::from_secret(access_secret.as_bytes()),
      &Validation::default(),
    )
    .map_err(|err| {
      if let ErrorKind::ExpiredSignature = err.kind() {
        AuthError::ExpiredSignature
      } else {
        AuthError::InvalidToken(err.to_string())
      }
    })?;

    Ok(token_data.claims)
  }
}

pub enum AuthError {
  InvalidToken(String),
  ExpiredSignature,
}

impl IntoResponse for AuthError {
  fn into_response(self) -> Response {
    let (status, error_message) = match self {
      AuthError::InvalidToken(ref reason) => (StatusCode::BAD_REQUEST, reason.as_str()),
      AuthError::ExpiredSignature => (StatusCode::UNAUTHORIZED, "Expired Signature"),
    };

    let body = Json(serde_json::json!({
        "error": error_message,
    }));

    (status, body).into_response()
  }
}
