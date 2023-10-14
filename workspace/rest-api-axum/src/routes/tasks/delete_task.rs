use crate::errors::CustomError;
use axum::extract::Path;
use axum::http::StatusCode;
use serde_json::json;
use serde_json::Value;

use crate::db::DatabaseConnection;
use axum::Json;

use crate::models::task;

pub async fn handler(
    DatabaseConnection(mut conn): DatabaseConnection,
    Path(id): Path<i32>,
) -> Result<(StatusCode, Json<Value>), CustomError> {
    let _find: task::Task = sqlx::query_as("SELECT * FROM task where id=$1")
        .bind(id)
        .fetch_one(&mut *conn)
        .await
        .map_err(|_| CustomError::TaskNotFound)?;

    sqlx::query("DELETE FROM task WHERE id=$1")
        .bind(id)
        .execute(&mut *conn)
        .await
        .map_err(|_| CustomError::TaskNotFound)?;

    Ok((StatusCode::OK, Json(json!({"msg": "Task Deleted"}))))
}
