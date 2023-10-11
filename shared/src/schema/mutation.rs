use super::models::user::UserModel;
use crate::database::repositories::user::UserRepository;
use async_graphql::{Context, Object, Result};
use sea_orm::DatabaseConnection;

pub struct Mutation;

#[Object]
impl Mutation {
  async fn signup<'ctx>(&self, ctx: &Context<'ctx>, name: String) -> Result<UserModel> {
    let conn = ctx.data::<DatabaseConnection>().unwrap();

    let user_repository = UserRepository::new(conn);

    let u = user_repository.create(name).await.unwrap();

    Ok(u)
  }
}
