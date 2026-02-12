use std::sync::{Arc, Mutex};

use axum::{
    Router,
    routing::{delete, get, post, put},
};
use tower_http::cors::CorsLayer;

use crate::{
    controller::{
        create_todo::create_todo, delete_todo::delete_todo, get_todo::get_todo,
        get_todos::get_todos, update_todo::update_todo, 
    },
    types::Db,
};

mod controller;
mod types;
mod middleware;

#[tokio::main]
async fn main() {
    std::fs::create_dir("data");
    let connection = sqlite::open("data/todo.db").unwrap();
    let db: Db = Arc::new(Mutex::new(connection));

    let app = Router::new()
        .route("/", get(|| async { "Welcome to the backend CRUD App!" }))
        .route("/todo", post(create_todo))
        .route("/todo", get(get_todos))
        .route("/todo/{id}", get(get_todo))
        .route("/todo/{id}", put(update_todo))
        .route("/todo/{id}", delete(delete_todo))
        .layer(CorsLayer::permissive())
        .with_state(db);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
