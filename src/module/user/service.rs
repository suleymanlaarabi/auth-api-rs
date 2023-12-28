use std::io::Error;

use crate::utils::file::json::read_json;

use super::data::user_types::User;

pub fn get_user_and_users_with_username(
    username: String,
) -> Result<Option<(User, Vec<User>)>, Error> {
    let users: Vec<User> = match read_json("users.json") {
        Ok(users) => users,

        Err(e) => return Err(e),
    };

    let user = users.iter().find(|user| user.username == username);

    match user {
        Some(user) => Ok(Some((user.clone(), users))), // Assurez-vous que User implémente Clone
        None => Ok(None),
    }
}

pub fn get_user_with_username(username: String) -> Result<Option<User>, Error> {
    return match get_user_and_users_with_username(username) {
        Ok(data) => match data {
            Some(user) => Ok(Some(user.0)),
            None => Ok(None),
        },
        Err(e) => Err(e),
    };
}
