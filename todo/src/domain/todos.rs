use crate::domain::todo::Todo;

pub struct Todos {
    pub list: Vec<Todo>,
}

impl Todos {
    pub fn new(list: Vec<Todo>) -> Self {
        Todos { list }
    }
}
