use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::types::{Db, Status, TodoResponse, UpdateTodoType};

pub async fn update_todo(
    Path(todo_id): Path<i64>,
    State(db): State<Db>,
    Json(payload): Json<UpdateTodoType>,
) -> impl IntoResponse {
    let mut connection = db.lock().unwrap();

    let query = "SELECT * FROM Todos where id = ?";
    let mut statement = connection.prepare(query).unwrap();
    statement.bind((1, todo_id)).unwrap();

    if let Ok(sqlite::State::Row) = statement.next() {
        let id = statement.read::<i64, _>("id").unwrap();
        let task = statement.read::<String, _>("task").unwrap();
        let status = statement.read::<String, _>("status").unwrap();

        let updated_task = payload.task.unwrap_or(task);

        let updated_status = payload.status.unwrap_or(status);
        let mut statement = connection
            .prepare("UPDATE Todos SET task = ?, status = ? where id = ?")
            .unwrap();
        statement.bind((1, updated_task.as_str())).unwrap();
        statement.bind((2, updated_status.as_str())).unwrap();
        statement.bind((3, todo_id)).unwrap();
        statement.next().unwrap();

        (
            StatusCode::OK,
            Json(
                serde_json::json!({"message": "Todo updated successfully", "data": TodoResponse {
                    id: id,
                    task: updated_task,
                    status: Status::from_str(&updated_status),
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
