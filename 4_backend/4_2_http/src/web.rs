use crate::db::DataBase;
use crate::db::Role;
use crate::db::User;
use axum::extract::Query;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use log::info;
use serde::Deserialize;

pub async fn create_tables(State(db): State<DataBase>) -> Result<(), AppError> {
    info!("create_tables");
    db.create_tables().await.map_err(Into::into)
}

pub async fn drop_tables(State(db): State<DataBase>) -> Result<(), AppError> {
    info!("drop_tables");
    db.drop_tables().await.map_err(Into::into)
}

pub async fn create_user(
    Query(UserName { name }): Query<UserName>,
    State(db): State<DataBase>,
) -> Result<(), AppError> {
    info!("create_user {name}");
    db.create_user(&name).await.map_err(Into::into)
}

pub async fn update_user(
    Path(id): Path<i32>,
    Query(UpdateUser { key, value }): Query<UpdateUser>,
    State(db): State<DataBase>,
) -> Result<(), AppError> {
    info!("update_user {id} {key} {value}");
    db.update_user(id, &key, &value).await.map_err(Into::into)
}

pub async fn add_role_to_user(
    Path(id): Path<i32>,
    Query(SlugValue { slug }): Query<SlugValue>,
    State(db): State<DataBase>,
) -> Result<(), AppError> {
    info!("add_role_to_user {id} {slug}");
    db.add_role_to_user(id, &slug).await.map_err(Into::into)
}

pub async fn delete_role_from_user(
    Path(id): Path<i32>,
    Query(SlugValue { slug }): Query<SlugValue>,
    State(db): State<DataBase>,
) -> Result<(), AppError> {
    info!("delete_role_from_user {id} {slug}");
    db.delete_role_from_user(id, &slug)
        .await
        .map_err(Into::into)
}

pub async fn get_users(State(db): State<DataBase>) -> Result<Json<Vec<User>>, AppError> {
    let users = db.get_users().await?;
    info!("get_users: {users:?}");

    Ok(Json(users))
}

pub async fn get_user_with_roles(
    Path(id): Path<i32>,
    State(db): State<DataBase>,
) -> Result<Json<(User, Vec<Role>)>, AppError> {
    let res = db.get_user_with_roles(id).await?;
    info!("get_user_with_roles {id} -> {res:?}");

    Ok(Json(res))
}

pub async fn delete_user(Path(id): Path<i32>, State(db): State<DataBase>) -> Result<(), AppError> {
    info!("delete user {id}");
    db.delete_user(id).await.map_err(Into::into)
}

pub async fn create_role(
    Query(CreateRole {
        slug,
        name,
        permissions,
    }): Query<CreateRole>,
    State(db): State<DataBase>,
) -> Result<(), AppError> {
    info!("create_role {slug} {name} {permissions}");
    db.create_role(&slug, &name, &permissions)
        .await
        .map_err(Into::into)
}

pub async fn update_role(
    Path(slug): Path<String>,
    Query(UpdateRole { key, value }): Query<UpdateRole>,
    State(db): State<DataBase>,
) -> Result<(), AppError> {
    info!("update_role {slug} {key} {value}");
    db.update_role(&slug, &key, &value)
        .await
        .map_err(Into::into)
}

pub async fn get_roles(State(db): State<DataBase>) -> Result<Json<Vec<Role>>, AppError> {
    let roles = db.get_roles().await?;
    info!("get_roles -> {roles:?}");

    Ok(Json(roles))
}

pub async fn get_role(
    Path(slug): Path<String>,
    State(db): State<DataBase>,
) -> Result<Json<Role>, AppError> {
    let role = db.get_role_by_slug(&slug).await?;
    info!("get_role {slug} -> {role:?}");

    Ok(Json(role))
}

pub async fn delete_role(
    Path(slug): Path<String>,
    State(db): State<DataBase>,
) -> Result<(), AppError> {
    info!("delete_role {slug}");
    db.delete_role(&slug).await.map_err(Into::into)
}

pub struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

#[derive(Deserialize, Debug)]
pub struct UpdateUser {
    key: String,
    value: String,
}

#[derive(Deserialize, Debug)]
pub struct UpdateRole {
    key: String,
    value: String,
}

#[derive(Deserialize, Debug)]
pub struct CreateRole {
    slug: String,
    name: String,
    permissions: String,
}

#[derive(Deserialize, Debug)]
pub struct UserName {
    name: String,
}

#[derive(Deserialize, Debug)]
pub struct SlugValue {
    slug: String,
}
