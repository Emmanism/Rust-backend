use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};
use sqlite::Connection;

pub type Db = Arc<Mutex<Connection>>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    pub task: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
    PENDING,
    DONE,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoResponse {
    pub id: i64,
    pub task: String,
    pub status: Status,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTodoType {
    pub task: Option<String>,
    pub status: Option<String>,
}

impl Status {
    pub fn as_str(&self) -> &'static str {
        match self {
            Status::PENDING => "PENDING",
            Status::DONE => "DONE",
        }
    }

    pub fn from_str(status: &str) -> Self {
        match status {
            "PENDING" => Status::PENDING,
            "DONE" => Status::DONE,
            _ => Status::PENDING,
        }
    }
}
