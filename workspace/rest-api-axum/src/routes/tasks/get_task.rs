use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json;

use crate::db::DatabaseConnection;
use crate::errors::CustomError;
use crate::models::task;

pub async fn handler(
    DatabaseConnection(mut conn): DatabaseConnection,
    Path(id): Path<i32>,
) -> Result<(StatusCode, Json<task::Task>), CustomError> {
    let sql = "SELECT * FROM task where id=$1".to_string();

    let task: task::Task = sqlx::query_as(&sql)
        .bind(id)
        .fetch_one(&mut *conn)
        .await
        .map_err(|_| CustomError::TaskNotFound)?;

    Ok((StatusCode::OK, Json(task)))
}
