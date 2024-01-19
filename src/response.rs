use serde::Serialize;

use crate::model::Todo;

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[derive(Serialize)]
pub struct TodoData {
    pub todo: Todo,
}

#[derive(Serialize)]
pub struct SingleTodoResponse {
    pub status: String,
    pub data: Todo,
}

#[derive(Serialize)]
pub struct TodoListResponse {
    pub status: String,
    pub count: usize,
    pub data: Vec<Todo>,
}
