pub mod user_types {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct User {
        username: String,
        password: String,
    }
    impl User {
        pub fn new(username: String, password: String) -> User {
            User { username, password }
        }
    }
}
