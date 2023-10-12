use crate::{
  database::entities::{prelude::User, user},
  error::SeaGraphError,
};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection, EntityTrait};
pub struct UserRepository<'r>(&'r DatabaseConnection);

impl<'r> UserRepository<'r> {
  pub fn new(conn: &'r DatabaseConnection) -> Self {
    Self(conn)
  }

  pub async fn create<'s>(
    &self,
    name: &'s str,
    password: &'s str,
  ) -> Result<user::Model, SeaGraphError> {
    let hash_password = bcrypt::hash(password.as_bytes(), 8)
      .map_err(|err| SeaGraphError::ExecutionError(err.to_string()))?;

    user::ActiveModel {
      name: Set(name.into()),
      password: Set(hash_password.into()),
      ..Default::default()
    }
    .insert(self.0)
    .await
    .map_err(|err| SeaGraphError::DatabaseSeaError(err.to_string()))
  }

  pub async fn get_user_by_id(&self, id: i32) -> Result<user::Model, SeaGraphError> {
    User::find_by_id(id)
      .one(self.0)
      .await
      .map_err(|err| SeaGraphError::DatabaseSeaError(err.to_string()))
      .and_then(|record| {
        record.ok_or(SeaGraphError::DatabaseRecordNotFoundError {
          table: "user".into(),
          col: "id".into(),
          value: id.to_string(),
        })
      })
  }
}
