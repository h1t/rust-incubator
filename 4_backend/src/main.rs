use async_graphql::{EmptySubscription, Schema};
use async_graphql_axum::GraphQL;
use axum::routing::get;
use axum::Router;
use log::info;
use std::net::SocketAddr;
use step_4::db::DataBase;
use step_4::graphql::{MutationRoot, QueryRoot};
use step_4::web;

#[tokio::main]
async fn main() {
    env_logger::init();

    let db_name = std::env::var("DB_NAME").unwrap_or_else(|_| ":memory:".to_string());
    let db = DataBase::connect(&format!("sqlite://{db_name}"))
        .await
        .unwrap();
    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(db)
        .finish();
    let app = Router::new().route("/", get(web::graphiql).post_service(GraphQL::new(schema)));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    info!("start listenening: {addr}");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
