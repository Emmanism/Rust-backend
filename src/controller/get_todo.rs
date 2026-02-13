use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::{
    middleware::api_response::{error, success},
    types::{Db, Status, TodoResponse},
};

/* pub async fn get_todo(Path(todo_id): Path<i64>, State(db): State<Db>) -> impl IntoResponse {
    let mut connection = db.lock().unwrap();

    let query = "SELECT * FROM Todos where id = ?";
    let mut statement = connection.prepare(query).unwrap();
    statement.bind((1, todo_id)).unwrap();

    if let Ok(sqlite::State::Row) = statement.next() {
        let id = statement.read::<i64, _>("id").unwrap();
        let task = statement.read::<String, _>("task").unwrap();
        let status = statement.read::<String, _>("status").unwrap();

        let todo_status = Status::from_str(&status);

        (
            StatusCode::OK,
            Json(
                serde_json::json!({"message": "Todo fetched successfully", "data": TodoResponse {
                    id: id,
                    task: task,
                    status: todo_status,
                }}),
            ),
        )
            .into_response()
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"message": "No todo found with that todo id"})),
        )
            .into_response()
    }
}
 */

pub async fn get_todo(Path(todo_id): Path<i64>, State(db): State<Db>) -> impl IntoResponse {
    let mut connection = match db.lock() {
        Ok(conn) => conn,
        Err(_) => {
            return error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to acquire database lock",
            );
        }
    };

    let query = "SELECT * FROM todos WHERE id = ?";
    let mut statement = match connection.prepare(query) {
        Ok(stmt) => stmt,
        Err(_) => return error(StatusCode::INTERNAL_SERVER_ERROR, "Failed to prepare query"),
    };

    if statement.bind((1, todo_id)).is_err() {
        return error(StatusCode::INTERNAL_SERVER_ERROR, "Failed to bind todo_id");
    }

    match statement.next() {
        Ok(sqlite::State::Row) => {
            let id = statement.read::<i64, _>("id").unwrap_or(todo_id);
            let task = statement.read::<String, _>("task").unwrap_or_default();
            let status_str = statement
                .read::<String, _>("status")
                .unwrap_or("PENDING".to_string());
            let todo_status = Status::from_str(&status_str);

            success(
                StatusCode::OK,
                "Todo fetched successfully",
                Some(TodoResponse {
                    id,
                    task,
                    status: todo_status,
                }),
            )
        }
        Ok(sqlite::State::Done) => error(StatusCode::NOT_FOUND, "No todo found with that ID"),
        Err(_) => error(StatusCode::INTERNAL_SERVER_ERROR, "Failed to execute query"),
    }
}
