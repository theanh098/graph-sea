use crate::database::entities;
use crate::schema::models::user::UserModel;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection, DbErr};
pub struct UserRepository<'r>(&'r DatabaseConnection);

impl<'r> UserRepository<'r> {
  pub fn new(conn: &'r DatabaseConnection) -> Self {
    Self(conn)
  }

  pub async fn create(&self, name: impl ToString) -> Result<UserModel, DbErr> {
    let r = entities::user::ActiveModel {
      name: Set(name.to_string()),
      ..Default::default()
    }
    .insert(self.0)
    .await?;

    Ok(UserModel::from(r))
  }
}
