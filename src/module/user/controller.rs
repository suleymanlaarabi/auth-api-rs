use crate::{module::user::data::user_types::User, utils::file::json::read_json};
use actix_web::{
    get, post,
    web::{self, Json},
    HttpResponse, Responder, Scope,
};
use std::fs;

#[get("")]
async fn get_all_user() -> impl Responder {
    let user1: User = User::new("".to_string(), "".to_string());
    HttpResponse::Ok().json(user1)
}

#[post("")]
async fn create_user(user: Json<User>) -> impl Responder {
    let mut users: Vec<User> = read_json("users.json").expect("Unable to get users");
    users.push(user.into_inner());

    let users_string = match serde_json::to_string(&users) {
        Ok(json) => json,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    match fs::write("users.json", &users_string.as_bytes()) {
        Ok(_) => HttpResponse::Ok().body("user saved"),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub fn user_controller() -> Scope {
    return web::scope("/users")
        .service(get_all_user)
        .service(create_user);
}
