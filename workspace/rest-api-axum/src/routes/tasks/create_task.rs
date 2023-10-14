use axum::http::StatusCode;
use axum::Json;

use crate::db::DatabaseConnection;
use crate::errors::CustomError;
use crate::models::task;

pub async fn handler(
    DatabaseConnection(mut conn): DatabaseConnection,
    Json(task): Json<task::NewTask>,
) -> Result<(StatusCode, Json<task::NewTask>), CustomError> {
    if task.task.is_empty() {
        return Err(CustomError::BadRequest);
    }

    let sql = "INSERT INTO task (task) values ($1)";

    let _ = sqlx::query(&sql)
        .bind(&task.task)
        .execute(&mut *conn)
        .await
        .map_err(|_| CustomError::InternalServerError)?;

    Ok((StatusCode::CREATED, Json(task)))
}
