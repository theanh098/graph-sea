use crate::database::entities::user::Model as UserEntity;
use async_graphql::SimpleObject;

#[derive(SimpleObject)]
pub struct UserModel {
  pub id: i32,

  pub name: String,

  #[graphql(visible = false)]
  pub password: String,
}

impl From<UserEntity> for UserModel {
  fn from(UserEntity { id, name, password }: UserEntity) -> Self {
    Self { id, name, password }
  }
}
