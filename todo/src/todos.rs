use crate::todo::Todo;

pub struct Todos {
    pub list: Vec<Todo>
}

impl Todos {
    pub fn new() -> Self {
        Todos {
            list: Vec::new()
        }
    }
    pub fn push_new_todo (&mut self, todo: Todo) {
        self.list.push(todo)
    }
}