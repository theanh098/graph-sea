use async_graphql::{Context, Object, Result};

use crate::database::repositories::user::UserRepository;
use crate::error::SeaGraphError;

use super::{models::user::UserModel, SeaGraphContext};

pub struct Query;

#[Object]
impl Query {
  async fn get_me<'ctx>(&self, ctx: &Context<'ctx>) -> Result<UserModel, SeaGraphError> {
    let claims = ctx.get_claims().await?;
    let conn = ctx.get_database_connection().await?;

    let user = UserRepository::new(conn).get_user_by_id(claims.id).await?;

    Ok(UserModel::from(user))
  }
}
