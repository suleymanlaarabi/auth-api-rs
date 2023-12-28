use actix_web::{App, HttpServer};
mod module;
use module::user;
mod utils;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(user::user_controller()))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
