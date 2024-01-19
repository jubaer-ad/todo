use crate::{
    model::{AppState, QueryOption, Todo, UpdateTodoSchema},
    response::{GenericResponse, SingleTodoResponse, TodoData, TodoListResponse},
};
use std::collections::HashMap;
// use crate::{model::*, response::*};
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use chrono::prelude::*;
use uuid::Uuid;

#[get("/isup")]
async fn check_server_up_or_not() -> impl Responder {
    const MESSAGE: &str = "বেঁচে আছি";
    let mut rsp = HashMap::new();
    rsp.insert("status", "success");
    rsp.insert("message", MESSAGE);
    let now = Local::now();
    let timestamp: &str = &now.format("%Y-%m-%d %H:%M:%S.%3f").to_string();
    rsp.insert("timestamp", timestamp);
    HttpResponse::Ok().json(rsp)
}

#[get("/todos")]
pub async fn todos_list_handler(
    opts: web::Query<QueryOption>,
    data: web::Data<AppState>,
) -> impl Responder {
    let todos = data.todo_db.lock().unwrap();

    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let todos: Vec<Todo> = todos.clone().into_iter().skip(offset).take(limit).collect();

    let rsp = TodoListResponse {
        status: "success".to_string(),
        count: todos.len(),
        data: todos,
    };
    HttpResponse::Ok().json(rsp)
}

#[post("/todos")]
async fn create_todo_handler(
    mut body: web::Json<Todo>,
    data: web::Data<AppState>,
) -> impl Responder {
    let mut vec = data.todo_db.lock().unwrap();
    let todo = vec.iter().find(|todo| todo.title == body.title);
    if todo.is_some() {
        let err_rsp = GenericResponse {
            status: "fail".to_string(),
            message: format!("Todo with title '{}' already exists", body.title),
        };

        return HttpResponse::Conflict().json(err_rsp);
    }

    let uuid_id = Uuid::new_v4();
    let datetime = Utc::now();

    body.id = Some(uuid_id.to_string());
    body.completed = Some(false);
    body.createdAt = Some(datetime);
    body.updatedAt = Some(datetime);

    let todo = body.to_owned();
    vec.push(body.into_inner());

    let rsp = &SingleTodoResponse {
        status: "success".to_string(),
        data: todo,
    };
    HttpResponse::Ok().json(rsp)
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(check_server_up_or_not)
        .service(todos_list_handler)
        .service(create_todo_handler);
    conf.service(scope);
}
