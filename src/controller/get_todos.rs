use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

use crate::{
    middleware::api_response::{error, success},
    types::{Db, Status, TodoResponse},
};

/* pub async fn get_todos(State(db): State<Db>) -> impl IntoResponse {
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
 */


 pub async fn get_todos(State(db): State<Db>) -> impl IntoResponse {
    let mut connection = match db.lock() {
        Ok(conn) => conn,
        Err(_) => {
            return error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to acquire database",
            )
        }
    };


    let mut statement = match connection.prepare(
        "SELECT id, task, status FROM todos",
    ) {
        Ok(stmt) => stmt,
        Err(_) => {
            return error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to prepare select query",
            )
        }
    };

    let mut todos: Vec<TodoResponse> = Vec::new();

   
    loop {
        match statement.next() {
            Ok(sqlite::State::Row) => {
                let id = statement
                    .read::<i64, _>("id")
                    .unwrap_or_default();

                let task = statement
                    .read::<String, _>("task")
                    .unwrap_or_default();

                let status_str = statement
                    .read::<String, _>("status")
                    .unwrap_or("PENDING".to_string());

                todos.push(TodoResponse {
                    id,
                    task,
                    status: Status::from_str(&status_str),
                });
            }
            Ok(sqlite::State::Done) => break, 
            Err(_) => {
                return error(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to fetch todos",
                )
            }
        }
    }

    success(
        StatusCode::OK,
        "Todos fetched successfully",
        Some(todos),
    )
}