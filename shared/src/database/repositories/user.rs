use crate::{
  database::entities::{prelude::User, user},
  error::AppError,
};
use sea_orm::{
  ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};
pub struct UserRepository<'r>(&'r DatabaseConnection);

impl<'r> UserRepository<'r> {
  pub fn new(conn: &'r DatabaseConnection) -> Self {
    Self(conn)
  }

  pub async fn create<'s>(
    &self,
    name: &'s str,
    password: &'s str,
  ) -> Result<user::Model, AppError> {
    let hash_password = bcrypt::hash(password.as_bytes(), 8)
      .map_err(|err| AppError::ExecutionError(err.to_string()))?;

    user::ActiveModel {
      name: Set(name.into()),
      password: Set(hash_password.into()),
      ..Default::default()
    }
    .insert(self.0)
    .await
    .map_err(|err| AppError::DatabaseSeaError(err.to_string()))
  }

  pub async fn get_user_by_id(&self, id: i32) -> Result<user::Model, AppError> {
    User::find_by_id(id)
      .one(self.0)
      .await
      .map_err(|err| AppError::DatabaseSeaError(err.to_string()))
      .and_then(|record| {
        record.ok_or(AppError::DatabaseRecordNotFoundError {
          table: "user".into(),
          col: "id".into(),
          value: id.to_string(),
        })
      })
  }

  pub async fn get_user_by_name(&self, name: String) -> Result<user::Model, AppError> {
    User::find()
      .filter(user::Column::Name.eq(name.as_str()))
      .one(self.0)
      .await
      .map_err(|err| AppError::DatabaseSeaError(err.to_string()))
      .and_then(|record| {
        record.ok_or(AppError::DatabaseRecordNotFoundError {
          table: "user".into(),
          col: "name".into(),
          value: name,
        })
      })
  }
}
