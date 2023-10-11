use async_graphql_axum::GraphQL;
use axum::{
  response::{self, IntoResponse},
  routing::get,
  Router, Server,
};
use shared::schema::{init_schema, GraphiQLSource};

#[tokio::main]
async fn main() {
  let schema = init_schema().await.unwrap_or_else(|err| panic!("{err}"));

  let app = Router::new().route("/", get(graphiql).post_service(GraphQL::new(schema)));

  println!("GraphiQL IDE: http://localhost:8000");

  Server::bind(&"127.0.0.1:8000".parse().unwrap())
    .serve(app.into_make_service())
    .await
    .unwrap();
}

async fn graphiql() -> impl IntoResponse {
  response::Html(GraphiQLSource::build().endpoint("/").finish())
}
