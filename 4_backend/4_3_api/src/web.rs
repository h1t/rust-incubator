use crate::{db::DataBase, db_thin_client::DBThinClient};
use crate::{Command, CommandResponse};
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use log::info;

pub async fn procces_command(
    State(db): State<DataBase>,
    Json(cmd): Json<Command>,
) -> Result<Json<CommandResponse>, AppError> {
    info!("process_command: {cmd:?}");

    let data = DBThinClient::new(db).execute_command(cmd).await?;

    Ok(Json(CommandResponse { data }))
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
