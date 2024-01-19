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
    let rsp = &GenericResponse {
        status: "success".to_string(),
        message: MESSAGE.to_string(),
    };
    HttpResponse::Ok().json(rsp);
}
