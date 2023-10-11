pub mod models;
mod mutation;
mod query;

pub use async_graphql::http::GraphiQLSource;
use async_graphql::{EmptySubscription, Schema as TSchema};

use sea_orm::{Database, DbErr};

pub type Schema = TSchema<query::Query, mutation::Mutation, EmptySubscription>;

pub async fn init_schema(
) -> Result<TSchema<query::Query, mutation::Mutation, EmptySubscription>, DbErr> {
  let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  let database_connection = Database::connect(db_url).await?;

  Ok(
    Schema::build(query::Query, mutation::Mutation, EmptySubscription)
      .data(database_connection)
      .finish(),
  )
}
