use sea_orm::DatabaseConnection;

pub struct UserRepository<'r>(&'r DatabaseConnection);

impl<'r> UserRepository<'r> {
  pub async fn create(&self) {}
}
