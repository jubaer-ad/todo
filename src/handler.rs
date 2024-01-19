// use crate::{
//     model::{AppState, QueryOption, Todo, UpdateTodoSchema},
//     response::{GenericResponse, SingleTodoResponse, TodoListResponse},
// };
use crate::{model::*, response::*};
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

#[post(/todos)]
async fn create_todo_handler(
    mut body: web::Json<Todo>,
    data: web::Data<AppState>,
) -> impl Responder {
    let mut vec = data.todo_db.lock().unwrap();
    let todo = vec.iter().find(|todo| todo.title == body.title);
    if todo.is_some() {
        let err_rsp = &GenericResponse {
            status: "fail".to_string(),
            message: format!("Todo with title '{}' already exists", body.title),
        };

        HttpResponse::Conflict().json(err_rsp)
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
        data: TodoData { todo },
    };
    HttpResponse::Ok().json(rs)
}
