use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
  extract::State,
  response::{self, IntoResponse},
  routing::get,
  Router, Server,
};
use dotenv::dotenv;
use shared::{
  schema::{init_schema, GraphiQLSource, Schema},
  security::authentication::Guard,
};

#[tokio::main]
async fn main() {
  dotenv().ok();

  tracing_subscriber::fmt().with_test_writer().init();
  let schema = init_schema().await;

  tokio::fs::write("schema.gql", schema.sdl()).await.unwrap();

  let app = Router::new().route("/", get(graphiql).post(graphql_handler).with_state(schema));

  println!("GraphiQL IDE: http://localhost:8000");

  Server::bind(&"127.0.0.1:8000".parse().unwrap())
    .serve(app.into_make_service())
    .await
    .unwrap();
}

async fn graphiql() -> impl IntoResponse {
  response::Html(GraphiQLSource::build().endpoint("/").finish())
}

async fn graphql_handler(
  guard: Guard,
  schema: State<Schema>,
  req: GraphQLRequest,
) -> GraphQLResponse {
  schema.execute(req.into_inner().data(guard)).await.into()
}
