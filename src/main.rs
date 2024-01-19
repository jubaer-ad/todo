pub mod handler;
pub mod model;
pub mod response;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{http::header, web, App, HttpServer};
use model::AppState;

// #[derive(Serialize)]
// pub struct GenericResponse {
//     pub status: String,
//     pub message: String,
// }

// #[get("/api/checkserverstatus")]
// async fn server_status_checker() -> impl Responder {
//     const MESSAGE: &str = "Actix server is working fine";
//     // let rsp_json = &GenericResponse {
//     //     status: "success".to_string(),
//     //     message: MESSAGE.to_string(),
//     // };

//     let mut rsp = HashMap::new();
//     rsp.insert("status", "success");
//     rsp.insert("message", MESSAGE);
//     let now = Local::now();
//     let timestamp: &str = &now.format("%Y-%m-%d %H:%M:%S.%3f").to_string();
//     rsp.insert("timestamp", timestamp);
//     HttpResponse::Ok().json(rsp)
// }
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actic_web=info");
    }
    env_logger::init();
    let todo_db = AppState::init();
    let app_data = web::Data::new(todo_db);
    println!("🚀 🦀 Server started successfully! 🦀 🚀");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_origin("http://localhost:3000/")
            .allowed_methods(vec!["GET", "POST", "DELETE", "PUT"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();
        App::new()
            .app_data(app_data.clone())
            .configure(handler::config)
            .wrap(cors)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
