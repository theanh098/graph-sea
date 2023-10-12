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

pub async fn init_schema() -> TSchema<query::Query, mutation::Mutation, EmptySubscription> {
  let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  let database_connection = Database::connect(db_url)
    .await
    .expect("Failed to connect to database");

  Schema::build(query::Query, mutation::Mutation, EmptySubscription)
    .data(database_connection)
    .finish()
}

#[async_trait]
pub trait SeaGraphContext {
  async fn get_claims(&self) -> Result<Claims, SeaGraphError>;
  async fn get_database_connection(&self) -> Result<&DatabaseConnection, SeaGraphError>;
}

#[async_trait]
impl<'ctx> SeaGraphContext for Context<'ctx> {
  async fn get_claims(&self) -> Result<Claims, SeaGraphError> {
    let guard = self
      .data::<OptionalGuard>()
      .map_err(|err| SeaGraphError::ExecutionError(err.message))?;

    match guard.into_inner() {
      None => Err(SeaGraphError::AuthenticationError),
      Some(claims) => Ok(claims),
    }
  }

  async fn get_database_connection(&self) -> Result<&DatabaseConnection, SeaGraphError> {
    let conn = self
      .data::<DatabaseConnection>()
      .map_err(|err| SeaGraphError::ExecutionError(err.message))?;

    Ok(conn)
  }
}
