use async_graphql::{Context, Object, Result};

use crate::database::repositories::user::UserRepository;
use crate::error::AppError;

use super::{models::user::UserModel, SeaGraphContext};

pub struct Query;

#[Object]
impl Query {
  #[graphql(name = "me")]
  async fn get_me<'ctx>(&self, ctx: &Context<'ctx>) -> Result<UserModel, AppError> {
    let claims = ctx.get_claims().await?;
    let conn = ctx.get_database_connection().await?;

    let user = UserRepository::new(conn).get_user_by_id(claims.id).await?;

    Ok(UserModel::from(user))
  }
}
