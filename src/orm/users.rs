use diesel::{prelude::SqliteConnection, RunQueryDsl};

use crate::schema::users;

use super::models::{NewUser, User};

pub fn create_user(conn: &SqliteConnection, username: &str, password: &str) -> usize {
    let new_user = NewUser { username, password };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn)
        .expect("Error saving new user")
}

pub fn get_users(conn: &SqliteConnection) -> Vec<User> {
    users::table
        .load::<User>(conn)
        .expect("Error loading users")
}
