use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

use crate::types::{Db, Status, Todo, TodoResponse};

pub async fn get_todos(State(db): State<Db>) -> impl IntoResponse {
    let mut connection = db.lock().unwrap();

    let mut todos: Vec<TodoResponse> = vec![];
    let query = "SELECT * FROM Todos";
    let mut statement = connection.prepare(query).unwrap();

    while let Ok(sqlite::State::Row) = statement.next() {
        let id = statement.read::<i64, _>("id").unwrap();
        let task = statement.read::<String, _>("task").unwrap();
        let status = statement.read::<String, _>("status").unwrap();

        let todo_status = Status::from_str(&status);
        todos.push(TodoResponse {
            id: id,
            task: task,
            status: todo_status,
        })
    }

    (
        StatusCode::OK,
        Json(serde_json::json!({"message": "Todo fetched successfully", "data": todos})),
    )
}
