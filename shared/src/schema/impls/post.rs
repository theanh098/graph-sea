use crate::database::repositories::post::PostRepository;
use crate::{
  error::AppError,
  schema::{
    models::{paginate::Paginate, post::PostModel},
    query::Query,
    SeaGraphContext,
  },
};
use async_graphql::Context;

impl Query {
  pub async fn impl_get_posts(
    ctx: &Context<'_>,
    take: i32,
    cursor: i32,
  ) -> Result<Paginate<PostModel>, AppError> {
    let db_conn = ctx.get_database_connection().await?;

    PostRepository::new(db_conn)
      .get_post_list(take, cursor)
      .await
      .map(|posts| {
        posts
          .into_iter()
          .map(|post| PostModel::from(post))
          .collect::<Vec<PostModel>>()
      })
      .map(|t| Paginate {
        nodes: t,
        count: 100,
        cursor: 3,
        has_next: false,
      })
  }
}
