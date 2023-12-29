use actix_web::{App, HttpServer};
mod module;
use module::user;
use orm::{db::establish_connection, users::create_user};
mod orm;
mod utils;

#[macro_use]
extern crate diesel;
extern crate dotenv;
pub mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let connection = &establish_connection();

    HttpServer::new(|| App::new().service(user::user_controller()))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
