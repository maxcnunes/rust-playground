use axum::http::StatusCode;
use axum::Json;

use crate::db::DatabaseConnection;
use crate::errors::CustomError;
use crate::models::task;

pub async fn handler(
    DatabaseConnection(mut conn): DatabaseConnection,
) -> Result<(StatusCode, Json<Vec<task::Task>>), CustomError> {
    let sql = "SELECT * FROM task ".to_string();

    let tasks = sqlx::query_as::<_, task::Task>(&sql)
        .fetch_all(&mut *conn)
        .await
        .map_err(|_| CustomError::InternalServerError)?;

    Ok((StatusCode::OK, Json(tasks)))
}
