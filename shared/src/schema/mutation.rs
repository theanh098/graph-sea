use super::{models::auth::AuthResponse, SeaGraphContext};
use crate::database::repositories::user::UserRepository;
use crate::error::SeaGraphError;
use async_graphql::{Context, Object, Result};

pub struct Mutation;

#[Object]
impl Mutation {
  #[graphql(name = "tokens")]
  async fn signup<'ctx>(
    &self,
    ctx: &Context<'ctx>,
    name: String,
    password: String,
  ) -> Result<AuthResponse, SeaGraphError> {
    let conn = ctx.get_database_connection().await?;
    let mut redis_connection = ctx.get_redis_connection().await?;

    let user = UserRepository::new(conn)
      .create(name.as_str(), password.as_str())
      .await?;

    Ok(AuthResponse::from_entity(user, &mut redis_connection).await?)
  }
}
