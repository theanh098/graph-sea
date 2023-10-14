mod impls;
pub mod models;
mod mutation;
mod query;

use crate::error::AppError;
use crate::security::authentication::{Claims, Guard};
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
  async fn get_claims(&self) -> Result<Claims, AppError>;
  async fn get_database_connection(&self) -> Result<&DatabaseConnection, AppError>;
  async fn get_redis_connection(&self) -> Result<deadpool_redis::Connection, AppError>;
}

#[async_trait]
impl<'ctx> SeaGraphContext for Context<'ctx> {
  async fn get_claims(&self) -> Result<Claims, AppError> {
    self
      .data::<Guard>()
      .map_err(|err| AppError::ExecutionError(err.message))
      .and_then(|guard| {
        guard
          .retrieve_claims()
          .ok_or(AppError::UnAuthorized(guard.retrieve_err_msg()))
      })
  }

  async fn get_database_connection(&self) -> Result<&DatabaseConnection, AppError> {
    self
      .data::<DatabaseConnection>()
      .map_err(|err| AppError::ExecutionError(err.message))
  }

  async fn get_redis_connection(&self) -> Result<deadpool_redis::Connection, AppError> {
    self
      .data::<deadpool_redis::Pool>()
      .map_err(|err| AppError::ExecutionError(err.message))
      .map(|pool| async {
        pool
          .get()
          .await
          .map_err(|err| AppError::ExecutionError(err.to_string()))
      })?
      .await
  }
}
