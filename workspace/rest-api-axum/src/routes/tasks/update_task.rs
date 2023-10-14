use axum::extract::Path;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use sqlx::PgPool;

use crate::errors::CustomError;
use crate::models::task;

pub async fn handler(
    // Example using State instead of our custom pool manager like in the other routes
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(task): Json<task::UpdateTask>,
) -> Result<(StatusCode, Json<task::UpdateTask>), CustomError> {
    let sql = "SELECT * FROM task where id=$1".to_string();

    let _find: task::Task = sqlx::query_as(&sql)
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|_| CustomError::TaskNotFound)?;

    let _ = sqlx::query("UPDATE task SET task=$1 WHERE id=$2")
        .bind(&task.task)
        .bind(id)
        .execute(&pool)
        .await;

    Ok((StatusCode::OK, Json(task)))
}
