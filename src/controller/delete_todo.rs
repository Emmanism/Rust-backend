use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::{
    middleware::api_response::{error, success},
    types::Db,
};


/* pub async fn delete_todo(Path(todo_id): Path<i64>, State(db): State<Db>) -> impl IntoResponse {
    let mut connection = db.lock().unwrap();

    let query = "SELECT * FROM Todos where id = ?";
    let mut statement = connection.prepare(query).unwrap();
    statement.bind((1, todo_id)).unwrap();

    if let Ok(sqlite::State::Row) = statement.next() {
        let mut statement = connection
            .prepare("DELETE FROM Todos where id = ?")
            .unwrap();
        statement.bind((1, todo_id));
        statement.next().unwrap();

        (
            StatusCode::OK,
            Json(serde_json::json!({"message": "Todo deleted successfully"})),
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



 pub async fn delete_todo(
    Path(todo_id): Path<i64>,
    State(db): State<Db>,
) -> impl IntoResponse {
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
        Ok(sqlite::State::Row) => {
            let mut delete_stmt = match connection.prepare(
                "DELETE FROM todos WHERE id = ?",
            ) {
                Ok(stmt) => stmt,
                Err(_) => {
                    return error(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Failed to prepare delete query",
                    )
                }
            };

            if delete_stmt.bind((1, todo_id)).is_err()
                || delete_stmt.next().is_err()
            {
                return error(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to delete todo",
                );
            }

            success::<()>(
                StatusCode::OK,
                "Todo deleted successfully",
                None,
            )
        }

        Ok(sqlite::State::Done) => {
            error(StatusCode::NOT_FOUND, "Todo not found")
        }

        Err(_) => {
            error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to execute select query",
            )
        }
    }
}