pub mod models;
mod mutation;
mod query;

use crate::error::SeaGraphError;
use crate::security::authentication::{Claims, OptionalGuard};
pub use async_graphql::http::GraphiQLSource;
use async_graphql::Context;
use async_graphql::{EmptySubscription, Schema as TSchema};

use axum::async_trait;
use sea_orm::{Database, DatabaseConnection};

pub type Schema = TSchema<query::Query, mutation::Mutation, EmptySubscription>;
pub type RedisConnection = deadpool_redis::Connection;

pub async fn init_schema() -> TSchema<query::Query, mutation::Mutation, EmptySubscription> {
  let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  let database_connection = Database::connect(db_url)
    .await
    .expect("Failed to connect to database");

  let redis_pool = deadpool_redis::Config::from_url("redis://127.0.0.1/")
    .create_pool(Some(deadpool_redis::Runtime::Tokio1))
    .expect("Failed to connect to redis server");

  Schema::build(query::Query, mutation::Mutation, EmptySubscription)
    .data(database_connection)
    .data(redis_pool)
    .finish()
}

#[async_trait]
pub trait SeaGraphContext {
  async fn get_claims(&self) -> Result<Claims, SeaGraphError>;
  async fn get_database_connection(&self) -> Result<&DatabaseConnection, SeaGraphError>;
  async fn get_redis_connection(&self) -> Result<deadpool_redis::Connection, SeaGraphError>;
}

#[async_trait]
impl<'ctx> SeaGraphContext for Context<'ctx> {
  async fn get_claims(&self) -> Result<Claims, SeaGraphError> {
    self
      .data::<OptionalGuard>()
      .map_err(|err| SeaGraphError::ExecutionError(err.message))
      .and_then(|guard| guard.into_inner().ok_or(SeaGraphError::AuthenticationError))
  }

  async fn get_database_connection(&self) -> Result<&DatabaseConnection, SeaGraphError> {
    self
      .data::<DatabaseConnection>()
      .map_err(|err| SeaGraphError::ExecutionError(err.message))
  }

  async fn get_redis_connection(&self) -> Result<deadpool_redis::Connection, SeaGraphError> {
    let pool = self
      .data::<deadpool_redis::Pool>()
      .map_err(|err| SeaGraphError::ExecutionError(err.message))?;

    pool
      .get()
      .await
      .map_err(|err| SeaGraphError::ExecutionError(err.to_string()))
  }
}
