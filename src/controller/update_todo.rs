use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::{
    middleware::api_response::{error, success},
    types::{Db, Status, TodoResponse, UpdateTodoType},
};


/* 
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
 */




pub async fn update_todo(
    Path(todo_id): Path<i64>,
    State(db): State<Db>,
    Json(payload): Json<UpdateTodoType>,
) -> impl IntoResponse {
    let task: String = match payload.task {
        Some(t) if !t.trim().is_empty() => t,
        _ => {
            return error(
                StatusCode::BAD_REQUEST,
                "Field 'task' is required",
            )
        }
    };

    let status_str = match payload.status {
        Some(s) => s,
        None => {
            return error(
                StatusCode::BAD_REQUEST,
                "Field 'status' is required",
            )
        }
    };


    let status = match status_str.as_str() {
        "PENDING" | "DONE" => Status::from_str(&status_str),
        _ => {
            return error(
                StatusCode::BAD_REQUEST,
                "Invalid status. Allowed values: PENDING, DONE",
            )
        }
    };


    let mut connection = match db.lock() {
        Ok(conn) => conn,
        Err(_) => {
            return error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to acquire database lock",
            )
        }
    };


    let mut select_stmt = match connection.prepare(
        "SELECT id FROM todos WHERE id = ?",
    ) {
        Ok(stmt) => stmt,
        Err(_) => {
            return error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to prepare select query",
            )
        }
    };

    if select_stmt.bind((1, todo_id)).is_err() {
        return error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to bind todo id",
        );
    }

    match select_stmt.next() {
        Ok(sqlite::State::Row) => {}

        Ok(sqlite::State::Done) => {
            return error(StatusCode::NOT_FOUND, "Todo not found");
        }
        Err(_) => {
            return error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to execute select query",
            )
        }
    }

    let mut update_stmt = match connection.prepare(
        "UPDATE todos SET task = ?, status = ? WHERE id = ?",
    ) {
        Ok(stmt) => stmt,
        Err(_) => {
            return error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to prepare update query",
            )
        }
    };

    if update_stmt.bind((1, task.as_str())).is_err()
        || update_stmt.bind((2, status.as_str())).is_err()
        || update_stmt.bind((3, todo_id)).is_err()
        || update_stmt.next().is_err()
    {
        return error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to update todo",
        );
    }

    success(
        StatusCode::OK,
        "Todo updated successfully",
        Some(TodoResponse {
            id: todo_id,
            task,
            status,
        }),
    )
}