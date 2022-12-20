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

    pub fn remove_todo (&mut self, index_todo: usize) {
        if self.list.get(index_todo).is_some() {
            self.list.remove(index_todo);
        } else {
            panic!("Index not found")
        }
    }
}