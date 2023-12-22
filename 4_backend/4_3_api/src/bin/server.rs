use axum::routing::post;
use axum::Router;
use log::info;
use step_4_3::db::DataBase;
use step_4_3::web;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    env_logger::init();

    let db_name = std::env::var("DB_NAME").unwrap_or_else(|_| ":memory:".to_string());
    let db = DataBase::connect(&format!("sqlite://{db_name}"))
        .await
        .unwrap();

    let app = Router::new()
        .route("/command", post(web::procces_command))
        .with_state(db);

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    info!("start listenening: {:?}", listener.local_addr());
    axum::serve(listener, app).await.unwrap();
}
