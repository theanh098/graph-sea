use crate::{
  database::entities::{
    post,
    prelude::{Post, User},
    user,
  },
  error::AppError,
};
use sea_orm::*;
pub type PostWithAuthor = (post::Model, Option<user::Model>);
pub struct PostRepository<'r>(&'r DatabaseConnection);

impl<'r> PostRepository<'r> {
  pub fn new(conn: &'r DatabaseConnection) -> Self {
    Self(conn)
  }

  pub async fn get_post_list(
    &self,
    take: i32,
    cursor: i32,
  ) -> Result<Vec<PostWithAuthor>, AppError> {
    Post::find()
      .find_also_related(User)
      .limit(Some(take as u64))
      .cursor_by(post::Column::Id)
      .after(cursor)
      .all(self.0)
      .await
      .map_err(|err| AppError::DatabaseSeaError(err.to_string()))
  }
}
