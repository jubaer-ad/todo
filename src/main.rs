pub mod model;
use chrono::Local;
use std::collections::HashMap;

use actix_web::middleware::Logger;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[get("/api/checkserverstatus")]
async fn server_status_checker() -> impl Responder {
    const MESSAGE: &str = "Actix server is working fine";
    // let rsp_json = &GenericResponse {
    //     status: "success".to_string(),
    //     message: MESSAGE.to_string(),
    // };

    let mut rsp = HashMap::new();
    rsp.insert("status", "success");
    rsp.insert("message", MESSAGE);
    let now = Local::now();
    let timestamp: &str = &now.format("%Y-%m-%d %H:%M:%S.%3f").to_string();
    rsp.insert("timestamp", timestamp);
    HttpResponse::Ok().json(rsp)
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actic_web=info");
    }
    env_logger::init();
    println!("🚀 🦀 Server started successfully! 🦀 🚀");

    HttpServer::new(move || {
        App::new()
            .service(server_status_checker)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
