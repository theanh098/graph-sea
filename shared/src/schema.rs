pub mod models;
mod mutation;
mod query;

pub use async_graphql::http::GraphiQLSource;
use async_graphql::{EmptySubscription, Schema};
use sea_orm::Database;

pub async fn init_schema() -> Schema<query::Query, mutation::Mutation, EmptySubscription> {
  let database_connection = Database::connect("postgresql://postgres:vitaminc@localhost:5432/sea")
    .await
    .unwrap();

  Schema::build(query::Query, mutation::Mutation, EmptySubscription)
    .data(database_connection)
    .finish()
}
