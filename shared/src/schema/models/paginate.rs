use async_graphql::{OutputType, SimpleObject};

#[derive(SimpleObject)]
pub struct Paginate<T: OutputType> {
  pub nodes: Vec<T>,
  pub count: i32,
  pub cursor: i32,
  pub has_next: bool,
}
