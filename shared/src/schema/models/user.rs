use async_graphql::SimpleObject;

#[derive(SimpleObject)]
pub struct User {
  pub id: i32,
  pub name: String,
}
