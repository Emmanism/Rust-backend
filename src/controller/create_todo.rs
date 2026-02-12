use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

use crate::{
    middleware::api_response::{error, success},
     types::{Db, Todo},
};
/* 
pub async fn create_todo(State(db): State<Db>, Json(payload): Json<Todo>) -> impl IntoResponse {
   let mut connection = db.lock().unwrap();

    let query = "
        CREATE TABLE IF NOT EXISTS Todos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            task TEXT NOT NULL,
            status TEXT NOT NULL CHECK(status in ('PENDING', 'DONE')) DEFAULT 'PENDING'
        );
    ";

    connection.execute(query).unwrap();

    let insert_query = "INSERT INTO Todos (task) VALUES (?)";
    let mut statement = connection.prepare(insert_query).unwrap();
    statement.bind((1, payload.task.as_str())).unwrap();
    statement.next().unwrap();

    (
        StatusCode::CREATED,
        Json(serde_json::json!({
            "message": "Todo created successfully"
        })),
    )
} */


pub async fn create_todo(
    State(db): State<Db>,
    Json(payload): Json<Todo>,
) -> impl IntoResponse {
    if payload.task.trim().is_empty() {
        return error(StatusCode::BAD_REQUEST, "Task is required");
    }


    let mut connection = match db.lock() {
        Ok(conn) => conn,
        Err(_) => {
            return error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Server Error",
            )
        }
    };

    let create_table_query = "
        CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            task TEXT NOT NULL,
            status TEXT NOT NULL CHECK(status in ('PENDING', 'DONE')) DEFAULT 'PENDING'
        );
    ";
    if connection.execute(create_table_query).is_err() {
        return error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to create todos table",
        );
    }


    let insert_query = "INSERT INTO todos (task) VALUES (?)";
    let mut statement = match connection.prepare(insert_query) {
        Ok(stmt) => stmt,
        Err(_) => {
            return error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to prepare insert statement",
            )
        }
    };

    if statement.bind((1, payload.task.as_str())).is_err() || statement.next().is_err() {
        return error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to insert todo",
        );
    }

  
    success::<()>(StatusCode::CREATED, "Todo created successfully", None)
}
