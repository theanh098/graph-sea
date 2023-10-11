pub mod models;
mod mutation;
mod query;

pub use async_graphql::http::GraphiQLSource;
use async_graphql::{EmptySubscription, Schema};
use dotenv::dotenv;
use sea_orm::{Database, DbErr};

pub async fn init_schema(
) -> Result<Schema<query::Query, mutation::Mutation, EmptySubscription>, DbErr> {
  dotenv().ok();

  let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  let database_connection = Database::connect(db_url).await?;

  Ok(
    Schema::build(query::Query, mutation::Mutation, EmptySubscription)
      .data(database_connection)
      .finish(),
  )
}
