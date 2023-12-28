use crate::{
    module::user::data::user_types::User,
    module::user::service::get_user_with_username,
    utils::file::json::{read_json, write_json},
};

use actix_web::{
    get, post,
    web::{self, Json},
    HttpResponse, Responder, Scope,
};

#[get("/{username}")]
async fn get_user(username: web::Path<String>) -> impl Responder {
    match get_user_with_username(username.into_inner()) {
        Ok(res) => match res {
            Some(user) => return HttpResponse::Ok().body(serde_json::to_string(&user).unwrap()),
            None => return HttpResponse::NotFound().body("No user found"),
        },
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };
}

#[post("")]
async fn create_user(user: Json<User>) -> impl Responder {
    let mut users: Vec<User> = match read_json("users.json") {
        Ok(users) => users,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let exist = users.iter().any(|f| f.username == user.username);

    if exist {
        return HttpResponse::InternalServerError().body("User already created");
    }

    users.push(user.into_inner());

    match write_json("users.json", users) {
        Ok(_) => HttpResponse::Ok().body("user saved"),
        Err(_) => return HttpResponse::InternalServerError().finish(),
    }
}

pub fn user_controller() -> Scope {
    return web::scope("/users").service(get_user).service(create_user);
}
