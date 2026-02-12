use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::types::{Db, Status, Todo, TodoResponse};

pub async fn get_todo(Path(todo_id): Path<i64>, State(db): State<Db>) -> impl IntoResponse {
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
