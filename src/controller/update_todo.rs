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
     if payload.task.is_none() || payload.status.is_none() {
        return error(
            StatusCode::BAD_REQUEST,
            "At least one of 'task' or 'status' must be provided",
        );
    }

    let mut connection = match db.lock() {
        Ok(conn) => conn,
        Err(_) => {
            return error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to acquire database lock",
            )
        }
    };

     let query = "SELECT id, task, status FROM todos WHERE id = ?";

    let mut select_stmt = match connection.prepare(
        query,
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
        return error(StatusCode::INTERNAL_SERVER_ERROR, "Failed to bind todo id");
    }

    let (current_task, current_status) = match select_stmt.next() {
        Ok(sqlite::State::Row) => {
            let task = select_stmt
                .read::<String, _>("task")
                .unwrap_or_default();

            let status = select_stmt
                .read::<String, _>("status")
                .unwrap_or("PENDING".to_string());

            (task, status)
        }
        Ok(sqlite::State::Done) => {
            return error(StatusCode::NOT_FOUND, "Todo not found");
        }
        Err(_) => {
            return error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to execute select query",
            )
        }
    };


    let updated_task = payload.task.unwrap_or(current_task);
    let updated_status = payload.status.unwrap_or(current_status);


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

    if update_stmt
        .bind((1, updated_task.as_str()))
        .is_err()
        || update_stmt
            .bind((2, updated_status.as_str()))
            .is_err()
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
            task: updated_task,
            status: Status::from_str(&updated_status),
        }),
    )
}