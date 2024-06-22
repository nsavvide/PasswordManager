#[derive(Clone)]
pub struct Password {
    id: usize,
    title: String,
    username: String,
    password: String
}

impl Password {
    pub fn new(title: String, username: String, password: String) -> Password {
        Password {
            id: 0,
            title,
            username,
            password
        }
    }

    pub fn new_with_id(id: usize, title: String, username: String, password: String) -> Password {
        Password {
            id,
            title,
            username,
            password
        }
    }
}