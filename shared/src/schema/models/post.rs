use async_graphql::SimpleObject;

use crate::database::repositories::post::PostWithAuthor;

use super::user::UserModel;

#[derive(SimpleObject)]
pub struct PostModel {
  id: i32,
  title: String,
  text: String,
  author: UserModel,
}

impl From<PostWithAuthor> for PostModel {
  fn from((post, author): PostWithAuthor) -> Self {
    Self {
      id: post.id,
      title: post.title,
      text: post.text,
      author: UserModel::from(author.unwrap()),
    }
  }
}
