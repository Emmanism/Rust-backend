use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

use crate::types::{Db, Todo, TodoResponse};

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
        Json(serde_json::json!({"message": "Todo created successfully"})),
    )
}
