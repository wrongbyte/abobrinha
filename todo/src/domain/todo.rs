use std::fmt;

use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct Todo {
    pub message: String,
    pub done: bool,
    pub id: Uuid,
}

impl Todo {
    pub fn new(message: String, id: Uuid) -> Self {
        Todo {
            message,
            done: false,
            id,
        }
    }
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let marker = if self.done { 'X' } else { ' ' };
        write!(f, "[{marker}] - {} - id: {}", self.message, self.id)?;
        Ok(())
    }
}

pub mod mocks {
    use crate::domain::todo::Todo;
    use crate::domain::todos::Todos;
    use uuid::Uuid;

    pub fn builder(number_todos: usize, done_todo: Option<usize>) -> Todos {
        let list: Vec<Todo> = (0..number_todos)
            .map(|index| {
                let id = Uuid::new_v4();
                let message = format!("todo {}", index);
                let mut todo = Todo::new(message, id);
                if let Some(done_index) = done_todo {
                    if index == done_index {
                        todo.done = true;
                    }
                }
                todo
            })
            .collect();
        Todos::new(list)
    }

    factori::factori!(Todos, {
        default {
            _list:Vec<Todo> = vec![],
            number_todos: usize = 0,
            done_todo: Option<usize> = None
        }
        builder {
            builder(number_todos, done_todo)
        }
    });
}
