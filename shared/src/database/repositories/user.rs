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

  pub async fn create(
    &self,
    name: impl ToString,
    password: impl ToString,
  ) -> Result<user::Model, SeaGraphError> {
    user::ActiveModel {
      name: Set(name.to_string()),
      password: Set(password.to_string()),
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
        record
          .ok_or(SeaGraphError::DatabaseRecordNotFoundError {
            table: "user".into(),
            col: "id".into(),
            value: id.to_string(),
          })
          .and_then(|user| Ok(user))
      })
  }
}
