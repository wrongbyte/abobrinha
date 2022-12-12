#[derive(Debug, Clone)]
pub struct Todo {
    pub message: String,
}

impl Todo {
    pub fn new(message: String) -> Self {
        Todo { message }
    }
}
