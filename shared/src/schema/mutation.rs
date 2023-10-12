use super::{models::user::UserModel, SeaGraphContext};
use crate::database::repositories::user::UserRepository;
use crate::error::SeaGraphError;
use async_graphql::{Context, Object, Result};

pub struct Mutation;

#[Object]
impl Mutation {
  async fn signup<'ctx>(
    &self,
    ctx: &Context<'ctx>,
    name: String,
    password: String,
  ) -> Result<UserModel, SeaGraphError> {
    let conn = ctx.get_database_connection().await?;

    let user = UserRepository::new(conn).create(name, password).await?;

    Err(SeaGraphError::AuthenticationError)
  }
}
