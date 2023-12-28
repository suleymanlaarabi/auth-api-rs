pub mod user_types {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Clone)]
    pub struct User {
        pub username: String,
        pub password: String,
    }
}
