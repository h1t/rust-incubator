use axum::routing::{delete, post};
use axum::{routing::get, Router};
use log::info;
use step_4_2::db::DataBase;
use step_4_2::web::{self, ApiDoc};
use tokio::net::TcpListener;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() {
    env_logger::init();

    let db_name = std::env::var("DB_NAME").unwrap_or_else(|_| ":memory:".to_string());
    let db = DataBase::connect(&format!("sqlite://{db_name}"))
        .await
        .unwrap();

    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/create_tables", post(web::create_tables))
        .route("/drop_tables", delete(web::drop_tables))
        .route(
            "/users/:id",
            get(web::get_user_with_roles)
                .delete(web::delete_user)
                .patch(web::update_user),
        )
        .route("/users", get(web::get_users).post(web::create_user))
        .route(
            "/users/roles/:id",
            post(web::add_role_to_user).delete(web::delete_role_from_user),
        )
        .route(
            "/roles/:slug",
            get(web::get_role)
                .patch(web::update_role)
                .delete(web::delete_role),
        )
        .route("/roles", get(web::get_roles).post(web::create_role))
        .with_state(db);

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    info!("start listenening: {:?}", listener.local_addr());
    axum::serve(listener, app).await.unwrap();
}
